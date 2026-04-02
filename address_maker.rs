use rand::Rng;
use hex::encode;

pub fn make_address() -> String {
    let mut buf = [0u8; 20];
    rand::thread_rng().fill(&mut buf);
    format!("0x{}", encode(buf))
}
