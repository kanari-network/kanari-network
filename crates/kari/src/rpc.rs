use futures::FutureExt;
use jsonrpc_core::{IoHandler, Params, Result as JsonRpcResult, Error as JsonRpcError};
use jsonrpc_http_server::{ServerBuilder, AccessControlAllowOrigin, DomainsValidation};
use serde_json::{json, Value as JsonValue, Value}; // Import Value from serde_json
use crate::blockchain::{BLOCKCHAIN, BALANCES};
use crate::CHAIN_ID;
use crate::wallet::send_coins;


// RPC server
fn get_latest_block(_params: Params) -> JsonRpcResult<JsonValue> {
    unsafe {
        if let Some(block) = BLOCKCHAIN.back() {
            Ok(serde_json::to_value(block).unwrap())
        } else {
            Ok(JsonValue::Null)
        }
    }
}

fn get_chain_id(_params: Params) -> JsonRpcResult<JsonValue> {
    Ok(JsonValue::String(CHAIN_ID.to_string()))
}

fn get_block_by_index(params: Params) -> JsonRpcResult<JsonValue> {
    let index: u32 = params.parse().map_err(|e| jsonrpc_core::Error::invalid_params(format!("Invalid index parameter: {}", e)))?;

    unsafe {
        if let Some(block) = BLOCKCHAIN.iter().find(|b| b.index == index) {
            Ok(serde_json::to_value(block).unwrap())
        } else {
            Ok(JsonValue::Null)
        }
    }
}

// New API: Get balance by address
fn get_balance(params: Params) -> JsonRpcResult<JsonValue> {
    let address: String = params.parse().map_err(|e| 
        jsonrpc_core::Error::invalid_params(format!("Invalid address parameter: {}", e))
    )?;

    let balances = unsafe { BALANCES.as_ref().unwrap().lock().unwrap() };
    let balance = balances.get(&address).cloned().unwrap_or(0);

    Ok(json!(balance))
}

// New API: Send Transaction
fn send_transaction(params: Params) -> JsonRpcResult<JsonValue> {
    let params: serde_json::Map<String, Value> = params.parse().map_err(|e| 
        jsonrpc_core::Error::invalid_params(format!("Invalid parameters: {}", e))
    )?;

    let sender = params.get("sender").and_then(|v| v.as_str()).ok_or(
        JsonRpcError::invalid_params("Missing 'sender' parameter")
    )?;

    let receiver = params.get("receiver").and_then(|v| v.as_str()).ok_or(
        JsonRpcError::invalid_params("Missing 'receiver' parameter")
    )?;

    let amount: u64 = params.get("amount").and_then(|v| v.as_u64()).ok_or(
        JsonRpcError::invalid_params("Missing or invalid 'amount' parameter")
    )?;

    if let Some(transaction) = send_coins(sender.to_string(), receiver.to_string(), amount) {
        // ... (Add logic to broadcast the transaction to the network)

        Ok(json!({ "message": "Transaction sent successfully", "transaction": transaction }))
    } else {
        Err(JsonRpcError::internal_error()) // Remove the string argument
    }
}

pub async fn start_rpc_server() {
    let mut io = IoHandler::new();

    io.add_method("get_latest_block", |params| {
        futures::future::ready(get_latest_block(params)).boxed()
    });

    io.add_method("get_chain_id", |params| {
        futures::future::ready(get_chain_id(params)).boxed()
    });

    io.add_method("get_block_by_index", |params| {
        futures::future::ready(get_block_by_index(params)).boxed()
    });

    io.add_method("get_balance", |params| {
        futures::future::ready(get_balance(params)).boxed()
    });

    io.add_method("send_transaction", |params| {
        futures::future::ready(send_transaction(params)).boxed()
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    println!("RPC server running on http://127.0.0.1:3030");
    server.wait();
}
