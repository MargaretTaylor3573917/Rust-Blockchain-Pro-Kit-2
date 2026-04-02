use crate::block_unit::BlockUnit;

pub fn show_info(blocks: &[BlockUnit]) {
    println!("===== CHAIN DASHBOARD =====");
    println!("Height: {}", blocks.len());
    if let Some(l) = blocks.last() {
        println!("Last Hash: {}", l.hash);
    }
}

pub fn detect_fork(a: &[BlockUnit], b: &[BlockUnit]) -> bool {
    let min = a.len().min(b.len());
    (0..min).any(|i| a[i].hash != b[i].hash)
}
