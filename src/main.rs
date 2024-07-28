mod wallet;
mod utils;

use std::io::{self, Write};
use web3::types::U256;
use wallet::Wallet;

#[tokio::main]
async fn main() {
    let mut wallet = Wallet::new();
    let testnet_url = "https://sepolia.infura.io/v3/5baff4d94a624341b63eca02b95a2b1c";

    loop {
        println!("1. Create new wallet");
        println!("2. Load wallet from file");
        println!("3. Save wallet to file");
        println!("4. Check balance");
        println!("5. Send transaction");
        println!("6. Get Counter value");
        println!("7. Increment Counter value");
        println!("0. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u8 = choice.trim().parse().unwrap();

        match choice {
            1 => {
                wallet = Wallet::new();
                println!("New wallet created. Address: {}", wallet.get_address());
            }
            2 => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                wallet = Wallet::load_from_file(path.trim());
                println!("Wallet loaded. Address: {}", wallet.get_address());
            }
            3 => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();
                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();
                wallet.save_to_file(path.trim());
                println!("Wallet saved.");
            }
            4 => {
                wallet.check_balance(testnet_url).await;
            }
            5 => {
                print!("Enter recipient address: ");
                io::stdout().flush().unwrap();
                let mut to = String::new();
                io::stdin().read_line(&mut to).unwrap();

                print!("Enter amount in wei: ");
                io::stdout().flush().unwrap();
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).unwrap();
                let amount: U256 = amount.trim().parse().unwrap();

                wallet.send_transaction(to.trim(), amount, testnet_url).await;
            }
            6 => {
                print!("Enter Counter contract address: ");
                io::stdout().flush().unwrap();
                let mut contract_address = String::new();
                io::stdin().read_line(&mut contract_address).unwrap();
                
                match wallet.get_counter(testnet_url, contract_address.trim()).await {
                    Ok(count) => println!("Current counter value: {}", count),
                    Err(e) => eprintln!("Error getting counter value: {:?}", e),
                }
            }
            7 => {
                print!("Enter Counter contract address: ");
                io::stdout().flush().unwrap();
                let mut contract_address = String::new();
                io::stdin().read_line(&mut contract_address).unwrap();
                
                match wallet.increment_counter(testnet_url, contract_address.trim()).await {
                    Ok(tx_hash) => println!("Increment transaction sent with hash: {:?}", tx_hash),
                    Err(e) => eprintln!("Error incrementing counter: {:?}", e),
                }
            }
            0 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
