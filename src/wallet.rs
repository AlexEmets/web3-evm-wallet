use secp256k1::{Secp256k1, SecretKey, PublicKey};

use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::fs;
use web3::types::{TransactionRequest, U256, U64, TransactionParameters, AccessList, Bytes, Address};
use web3::transports::Http;
use web3::signing::{Key, SecretKeyRef};
use web3::contract::{Contract, Options};
use std::str::FromStr;
use web3::types::H256;
use web3::Error as Web3Error;
use crate::utils;

pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
}

#[derive(Serialize, Deserialize)]
pub struct KeyStorage {
    pub private_key: String,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (private_key, public_key) = secp.generate_keypair(&mut rng);
        
        Wallet { private_key, public_key }
    }

    pub fn get_address(&self) -> String {
        let public_key = self.public_key.serialize_uncompressed();
        let hash = utils::keccak256(&public_key[1..]);
        hex::encode(&hash[12..])
    }

    pub fn save_to_file(&self, file_path: &str) {
        let key_storage = KeyStorage {
            private_key: hex::encode(&self.private_key[..]),
            address: self.get_address(),
        };
        let data = serde_json::to_string(&key_storage).unwrap();
        fs::write(file_path, data).expect("Unable to write file");
    }

    pub fn load_from_file(file_path: &str) -> Self {
        let data = fs::read_to_string(file_path).expect("Unable to read file");
        let key_storage: KeyStorage = serde_json::from_str(&data).unwrap();
        let private_key = SecretKey::from_slice(&hex::decode(&key_storage.private_key).unwrap()).unwrap();
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        Wallet { private_key, public_key }
    }

    pub async fn check_balance(&self, web3_url: &str) {
        let http = Http::new(web3_url).unwrap();
        let web3 = web3::Web3::new(http);

        let address = web3::types::H160::from_slice(&hex::decode(self.get_address()).unwrap());
        let balance = web3.eth().balance(address, None).await.unwrap();
        println!("Balance: {} wei", balance);
    }

    pub async fn send_transaction(&self, to: &str, amount: U256, web3_url: &str) {
        let http = Http::new(web3_url).unwrap();
        let web3_http = web3::Web3::new(http);

        let tx = TransactionParameters {
            nonce: None,
            to: Some(web3::types::H160::from_slice(&hex::decode(to).unwrap())),
            gas: U256::from(21000),
            gas_price: None,
            value: amount,
            data: Bytes::from(Vec::new()), //?
            chain_id: None,
            transaction_type: None,
            access_list: Some(AccessList::default()), 
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };

        let signed_tx = web3_http.accounts().sign_transaction(tx, &self.private_key).await.unwrap();
        let result = web3_http.eth().send_raw_transaction(signed_tx.raw_transaction).await.unwrap();
        println!("Transaction sent with hash: {:?}", result);
    }

    pub async fn get_counter(&self, web3_url: &str, contract_address: &str) -> Result<U256, Box<dyn std::error::Error>> {
        let http = Http::new(web3_url)?;
        let web3 = web3::Web3::new(http);
        
        let abi = include_bytes!("contracts_abi/counter_abi.json");
        let contract_address = Address::from_str(contract_address)?;
        let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

        let count: U256 = contract.query("getCount", (), None, Options::default(), None).await?;
        Ok(count)
    }

    pub async fn increment_counter(&self, web3_url: &str, contract_address: &str) -> Result<H256, Box<dyn std::error::Error>> {
        let http = Http::new(web3_url)?;
        let web3 = web3::Web3::new(http);
        
        let abi = include_bytes!("contracts_abi/counter_abi.json");
        let contract_address = Address::from_str(contract_address)?;
        let contract = Contract::from_json(web3.eth(), contract_address, abi)?;
        
        let options = Options::default();
        let increment_tx = contract.signed_call("increment", (), options, &self.private_key).await?;
        
        // Перевірка статусу транзакції
        loop {
            if let Some(receipt) = web3.eth().transaction_receipt(increment_tx).await? {
                if receipt.status == Some(U64::from(1)) {
                    return Ok(increment_tx);
                } else {
                    return Err("Transaction failed".into());
                }
            }
            // В очікуванні підтвердження транзакції
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    

}
