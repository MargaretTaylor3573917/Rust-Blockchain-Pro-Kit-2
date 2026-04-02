use sha2::{Sha256, Digest};
use hex::encode;

pub fn build_root(mut hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return encode(Sha256::digest("empty"));
    }
    while hashes.len() > 1 {
        let mut next = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let left = &hashes[i];
            let right = hashes.get(i+1).unwrap_or(left);
            let hash = encode(Sha256::digest((left.clone() + right).as_bytes()));
            next.push(hash);
        }
        hashes = next;
    }
    hashes[0].clone()
}
