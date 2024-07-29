mod wallet;
mod utils;
mod contracts_manager;
mod smart_contract;

use std::io::{self, Write};
use web3::types::U256;
use wallet::Wallet;
use contracts_manager::ContractsManager;


#[tokio::main]
async fn main() {
    let mut wallet = Wallet::new();

    loop {
        println!("1. Create new wallet");
        println!("2. Load wallet from file");
        println!("3. Save wallet to file");
        println!("4. Check balance");
        println!("5. Send transaction");
        println!("6. Smart Contract execution");
        println!("7. Get Counter value");
        println!("8. Increment Counter value");
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
                wallet.check_balance().await;
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

                wallet.send_transaction(to.trim(), amount).await;
            }
            6 => {
                println!("This wallet supports following contracts: ");
                let sc_list = wallet.sc_manager.get_contracts_list();

                for sc in sc_list.iter() {
                    // TODO implement Display trait
                    println!("\n--------");
                    println!("Name : {}", sc.get_name());
                    print!("Functions : ");
                    let funcs_for_sc = sc.get_functions();
                    for func in funcs_for_sc {
                        print!("{} ", func);
                    }
                    println!("\n--------");
                }

                // TODO read SC name and SC function and then call it with specific params
            }
            7 => {
                print!("Enter Counter contract address: ");
                io::stdout().flush().unwrap();
                let mut contract_address = String::new();
                io::stdin().read_line(&mut contract_address).unwrap();
                
                match wallet.get_counter(contract_address.trim()).await {
                    Ok(count) => println!("Current counter value: {}", count),
                    Err(e) => eprintln!("Error getting counter value: {:?}", e),
                }
            }
            8 => {
                print!("Enter Counter contract address: ");
                io::stdout().flush().unwrap();
                let mut contract_address = String::new();
                io::stdin().read_line(&mut contract_address).unwrap();
                
                match wallet.increment_counter(contract_address.trim()).await {
                    Ok(tx_hash) => println!("Increment transaction sent with hash: {:?}", tx_hash),
                    Err(e) => eprintln!("Error incrementing counter: {:?}", e),
                }
            }
            0 => break,
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
