use sha2::{Sha256, Digest};
use hex::encode;

#[derive(Clone, Debug)]
pub struct BlockUnit {
    pub index: u64,
    pub prev_hash: String,
    pub time: u64,
    pub data: String,
    pub nonce: u64,
    pub hash: String,
}

impl BlockUnit {
    pub fn make_hash(&self) -> String {
        let raw = format!(
            "{}{}{}{}{}",
            self.index, self.prev_hash, self.time, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        encode(hasher.finalize())
    }
}
