use sha2::{Sha256, Digest};
use hex::encode;

#[derive(Debug, Clone)]
pub struct TxItem {
    pub txid: String,
    pub sender: String,
    pub receiver: String,
    pub value: f64,
    pub time: u64,
}

impl TxItem {
    pub fn build(sender: String, receiver: String, value: f64) -> Self {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let data = format!("{}{}{}{}", sender, receiver, value, time);
        let txid = encode(Sha256::digest(data.as_bytes()));
        TxItem { txid, sender, receiver, value, time }
    }
}
