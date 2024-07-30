use crate::smart_contract::SmartContract;
use std::fs;
use serde_json::Value;

pub struct ContractsManager {
    pub contracts_list:Vec<SmartContract>
} 

impl ContractsManager {
    pub const CONFIG_PATH: &'static str = "/home/oleksandryemets/Documents/Studying/CryptoWallet/src/smart_contracts_info.json";
}

impl ContractsManager {
    pub fn new() -> Self {
        let contracts_list = ContractsManager::get_smart_contracts_list();
        ContractsManager { contracts_list }
    }

    pub fn get_contracts_list(&self) -> &Vec<SmartContract> {
        &self.contracts_list
    }

    pub fn get_interface_by_contract_name(&self, name: &str) -> Option<&SmartContract> {
        self.contracts_list.iter().find(|contract| contract.get_name() == name)
    }

    // Parse `smart_contracts_info` config fileto extract all SC info. Used generally in constructor/new method 
    fn get_smart_contracts_list() -> Vec<SmartContract> {
        let config_data = fs::read_to_string(ContractsManager::CONFIG_PATH)
            .expect("Unable to read smart contracts configuration file.");
        let json_value: Value = serde_json::from_str(&config_data)
            .expect("JSON was not well-formatted.");
    
        let mut result_list: Vec<SmartContract> = Vec::new();
        
        if let Value::Array(contracts) = json_value {
            for contract in contracts {
                let name = contract["name"].as_str().unwrap().to_string();
                let address = contract["address"].as_str().unwrap().to_string();
                let abi_path = contract["abi_path"].as_str().unwrap().to_string();
    
                result_list.push(SmartContract::new(name, address, abi_path));
            }
        }
    
        result_list
    }

}