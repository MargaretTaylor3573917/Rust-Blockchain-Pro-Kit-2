use std::collections::HashMap;

pub struct ContractKV {
    data: HashMap<String, HashMap<String, String>>,
}

impl ContractKV {
    pub fn new() -> Self {
        ContractKV { data: HashMap::new() }
    }

    pub fn put(&mut self, contract: &str, key: &str, value: &str) {
        self.data.entry(contract.into())
            .or_default()
            .insert(key.into(), value.into());
    }

    pub fn get(&self, contract: &str, key: &str) -> Option<String> {
        self.data.get(contract)?.get(key).cloned()
    }
}
