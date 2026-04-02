use std::collections::HashMap;

pub struct NativeToken {
    pub name: &'static str,
    pub symbol: &'static str,
    pub supply: f64,
    holders: HashMap<String, f64>,
}

impl NativeToken {
    pub fn launch() -> Self {
        let mut holders = HashMap::new();
        holders.insert("ROOT".into(), 21_000_000.0);
        NativeToken {
            name: "RustNativeCoin",
            symbol: "RNC",
            supply: 21_000_000.0,
            holders,
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amt: f64) -> bool {
        let bal = *self.holders.get(from).unwrap_or(&0.0);
        if bal < amt { return false; }
        *self.holders.entry(from.into()).or_insert(0.0) -= amt;
        *self.holders.entry(to.into()).or_insert(0.0) += amt;
        true
    }
}
