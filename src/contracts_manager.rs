use crate::smart_contract::SmartContract;

pub struct ContractsManager {
    pub contracts_list:Vec<SmartContract>
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

    // use only in constructor/new method 
    fn get_smart_contracts_list() -> Vec<SmartContract> {
        let mut result_list: Vec<SmartContract> = Vec::new();
    
        result_list.push(SmartContract::new(
            "Counter".to_string(),
            "0xc6BCF9F0eaD0291e9e6D0cbD4aA4ca4Fa751707B".to_string(),
            "/home/oleksandryemets/Documents/Studying/CryptoWallet/src/contracts/ABIs/counter_abi.json".to_string()
        ));
        result_list.push(SmartContract::new(
            "Voting".to_string(),
            "address".to_string(),
            "/home/oleksandryemets/Documents/Studying/CryptoWallet/src/contracts/ABIs/voting.json".to_string()
        ));
        result_list
    }

}