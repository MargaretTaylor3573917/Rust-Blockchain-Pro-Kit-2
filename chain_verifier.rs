use crate::block_unit::BlockUnit;

pub fn verify_chain(blocks: &[BlockUnit]) -> bool {
    for i in 1..blocks.len() {
        let curr = &blocks[i];
        let prev = &blocks[i-1];
        if curr.hash != curr.make_hash() { return false; }
        if curr.prev_hash != prev.hash { return false; }
    }
    true
}
