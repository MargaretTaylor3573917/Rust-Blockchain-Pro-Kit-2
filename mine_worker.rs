use crate::block_unit::BlockUnit;

const DIFFICULTY: usize = 4;

pub fn start_mining(mut block: BlockUnit) -> BlockUnit {
    let target = "0".repeat(DIFFICULTY);
    loop {
        let h = block.make_hash();
        if h.starts_with(&target) {
            block.hash = h;
            return block;
        }
        block.nonce += 1;
    }
}
