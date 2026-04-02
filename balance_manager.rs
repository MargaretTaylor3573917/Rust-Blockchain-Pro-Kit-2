use std::collections::HashMap;

pub struct BalanceManager {
    map: HashMap<String, f64>,
}

impl BalanceManager {
    pub fn new() -> Self {
        BalanceManager { map: HashMap::new() }
    }

    pub fn get(&self, addr: &str) -> f64 {
        *self.map.get(addr).unwrap_or(&0.0)
    }

    pub fn send(&mut self, from: &str, to: &str, amount: f64) -> bool {
        if self.get(from) < amount { return false; }
        *self.map.entry(from.into()).or_insert(0.0) -= amount;
        *self.map.entry(to.into()).or_insert(0.0) += amount;
        true
    }
}
