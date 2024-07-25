use secp256k1::{Secp256k1, SecretKey, PublicKey};

use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use std::fs;
use web3::types::{TransactionRequest, U256, TransactionParameters, AccessList, Bytes};
use web3::transports::Http;
use web3::signing::{Key, SecretKeyRef};
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
        let web3 = web3::Web3::new(http);

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

        let signed_tx = web3.accounts().sign_transaction(tx, &self.private_key).await.unwrap();
        let result = web3.eth().send_raw_transaction(signed_tx.raw_transaction).await.unwrap();
        println!("Transaction sent with hash: {:?}", result);
    }
}
