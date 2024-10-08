use std::io;
use colored::Colorize;
use key::{generate_karix_address, list_wallet_files, load_wallet, save_wallet, send_coins};
use k2::blockchain::{load_blockchain, BALANCES};
use std::process::exit;


pub fn handle_keytool_command() -> Option<String> {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if any arguments were provided
    if args.len() > 2 {
        // Collect command line arguments
        let command = &args[2];
        // Use string comparison in the match statement
        match command.as_str() {
            "generate" => { 
                println!("Enter mnemonic length (12 or 24):");
                let mut mnemonic_length_str = String::new();
                io::stdin().read_line(&mut mnemonic_length_str).unwrap();
                let mnemonic_length: usize = mnemonic_length_str.trim().parse().expect("Invalid input");

                let (private_key, public_address, seed_phrase) = generate_karix_address(mnemonic_length);
                println!("New address generated:");
                println!("Private Key: {}", private_key.green());
                println!("Public Address: {}", public_address.green());
                println!("Seed Phrase: {}", seed_phrase.green());

                save_wallet(&public_address, &private_key, &seed_phrase);

                return Some(public_address);
            },
            "balance" => { // String comparison for "balance"
                println!("Enter public address:");
                let mut public_address = String::new();
                io::stdin().read_line(&mut public_address).unwrap();
                let public_address = public_address.trim().to_string();

                load_blockchain();

                let balance = unsafe {
                    BALANCES.as_ref().unwrap().lock().unwrap().get(&public_address).cloned().unwrap_or(0)
                };

                println!("Balance for {}: {}", public_address.green(), balance.to_string().green());
                return None; // Return None to indicate no address to be used further
            },
            "wallet" => { // String comparison for "wallet"
                println!("Enter public address to load:");
                let mut public_address = String::new();
                io::stdin().read_line(&mut public_address).unwrap();
                let public_address = public_address.trim().to_string();

                if let Some(wallet_data) = load_wallet(&public_address) {
                    println!("Wallet loaded:");
                    println!("Address: {}", wallet_data["address"].as_str().unwrap().green());
                    println!("Private Key: {}", wallet_data["private_key"].as_str().unwrap().green());
                    println!("Seed Phrase: {}", wallet_data["seed_phrase"].as_str().unwrap().green());
                    return Some(public_address);
                } else {
                    println!("Wallet not found for address: {}", public_address.red());
                    return None; // Return None to indicate no address to be used further
                }
            },
            "send" => { // String comparison for "send"
                // Prompt for sender's public address
                println!("Enter sender public address:");
                let mut sender_address = String::new();
                io::stdin().read_line(&mut sender_address).unwrap();
                let sender_address = sender_address.trim().to_string();
            
                // Prompt for receiver's public address
                println!("Enter receiver public address:");
                let mut receiver_address = String::new();
                io::stdin().read_line(&mut receiver_address).unwrap();
                let receiver_address = receiver_address.trim().to_string();
            
                // Prompt for the amount to send
                println!("Enter amount to send:");
                let mut amount_str = String::new();
                io::stdin().read_line(&mut amount_str).unwrap();
                let amount: u64 = amount_str.trim().parse().expect("Invalid input");
            
                // Load the blockchain (assuming this function is defined elsewhere)
                load_blockchain();
            
                // Call the send_coins function
                let transaction = send_coins(sender_address, receiver_address, amount);
            
                // Check if the transaction was successful
                if let Some(transaction) = transaction {
                    println!("Transaction successful:");
                    println!("Sender: {}", transaction.sender.green());
                    println!("Receiver: {}", transaction.receiver.green());
                    println!("Amount: {}", transaction.amount.to_string().green());
                    println!("Gas Cost: {}", transaction.gas_cost.to_string().green());
                } else {
                    println!("Transaction failed. Please check the following:");
                    println!("- Ensure the sender's address is correct and has sufficient funds.");
                    println!("- Ensure the receiver's address is correct.");
                    println!("- Ensure the amount is valid and within the sender's balance.");
                }
            
                // Return None to indicate no address to be used further
                None
            },
            "list" => { // String comparison for "list"}
                match list_wallet_files() {
                    Ok(wallets) => {
                        println!("Wallet files:");
                        for wallet in wallets {
                            println!("{}", wallet.green());
                        }
                    },
                    Err(e) => {
                        println!("Failed to list wallet files: {}", e);
                    }
                }
                return None;
            },
            _ => {
                println!("{}", "Invalid command".red());
                println!("Usage: kari keytool <command> [options]");
                println!("Commands:");
                println!("  {} - Generate new address", "generate".green());
                println!("  {} - Check balance", "balance".green());
                println!("  {} - Load existing wallet", "wallet".green());
                println!("  {} - Send coins", "send".green());
                println!("  {} - List wallet files", "list".green());
                exit(1); // Exit with an error code
        
            },
        }
    } else {
        // No command provided, print usage
        println!("Usage: kari keytool <command> [options]");
        println!("Commands:");
        println!("  {} - Generate new address", "generate".green());
        println!("  {} - Check balance", "balance".green());
        println!("  {} - Load existing wallet", "wallet".green());
        println!("  {} - Send coins", "send".green());
        println!("  {} - List wallet files", "list".green());
        exit(1); // Exit with an error code
    }
}
