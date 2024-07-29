use secp256k1::{Secp256k1, SecretKey, PublicKey};

use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::fs;
use web3::types::{U256, U64, TransactionParameters, AccessList, Bytes, Address};
use web3::transports::Http;
use web3::contract::{Contract, Options};
use std::str::FromStr;
use web3::types::H256;

use crate::utils;
use crate::contracts_manager::ContractsManager;

const WEB_URL:&str = "https://sepolia.infura.io/v3/5baff4d94a624341b63eca02b95a2b1c";


pub struct Wallet {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    pub web_url: String,
    pub sc_manager: ContractsManager
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
        let web_url = WEB_URL;

        Wallet { private_key, public_key, web_url:web_url.to_string(), sc_manager:ContractsManager::new() }
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
        Wallet { private_key, public_key, web_url:WEB_URL.to_string(), sc_manager:ContractsManager::new() }
    }

    pub async fn check_balance(&self) {
        let http = Http::new(&self.web_url).unwrap();
        let web3 = web3::Web3::new(http);

        let address = web3::types::H160::from_slice(&hex::decode(self.get_address()).unwrap());
        let balance = web3.eth().balance(address, None).await.unwrap();
        println!("Balance: {} wei", balance);
    }

    pub async fn send_transaction(&self, to: &str, amount: U256) {
        let http = Http::new(&self.web_url).unwrap();
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

    pub async fn get_counter(&self, contract_address: &str) -> Result<U256, Box<dyn std::error::Error>> {
        let http = Http::new(&self.web_url)?;
        let web3 = web3::Web3::new(http);
        
        let abi = include_bytes!("contracts/ABIs/counter_abi.json");
        let contract_address = Address::from_str(contract_address)?;
        let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

        let count: U256 = contract.query("getCount", (), None, Options::default(), None).await?;
        Ok(count)
    }

    pub async fn increment_counter(&self, contract_address: &str) -> Result<H256, Box<dyn std::error::Error>> {
        let http = Http::new(&self.web_url)?;
        let web3 = web3::Web3::new(http);
        
        let abi = include_bytes!("contracts/ABIs/counter_abi.json");
        let contract_address = Address::from_str(contract_address)?;
        let contract = Contract::from_json(web3.eth(), contract_address, abi)?;
        
        let options = Options::default();
        let confirmations = 3; 
    
        let receipt = contract.signed_call_with_confirmations("increment", (), options,  confirmations, &self.private_key,).await?;
        
        if receipt.status == Some(U64::from(1)) {
            Ok(receipt.transaction_hash)
        } else {
            Err("Transaction failed".into())
        }
    }
    
    
    

}
