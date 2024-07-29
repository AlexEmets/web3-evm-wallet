use serde_json::Value;
use std::fs;

pub struct SmartContract {
    name: String,
    address: String,
    abi_path: String,
    abi: Option<serde_json::Value>,
}

impl SmartContract {
    // Створення нового смарт-контракту з ABI
    pub fn new(name: String, address: String, abi_path: String) -> Self {
        let abi = SmartContract::load_abi(&abi_path);
        SmartContract {
            name,
            address,
            abi_path,
            abi,
        }
    }

    // Завантаження ABI з файлу
    fn load_abi(path: &str) -> Option<Value> {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).ok(),
            Err(_) => None,
        }
    }

    // Отримання функцій контракту
    pub fn get_functions(&self) -> Vec<String> {
        if let Some(abi) = &self.abi {
            if let Some(functions) = abi.as_array() {
                return functions
                    .iter()
                    .filter_map(|item| {
                        if let Some(obj) = item.as_object() {
                            if obj.get("type") == Some(&Value::String("function".to_string())) {
                                return obj.get("name").and_then(Value::as_str).map(String::from);
                            }
                        }
                        None
                    })
                    .collect();
            }
        }
        vec![]
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    // fn call_function(&self, function_name: &str, params: Value) -> Result<Value, String> {
        
    // }
}