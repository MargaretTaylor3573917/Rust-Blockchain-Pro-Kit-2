use crate::block_unit::BlockUnit;
use std::time::SystemTime;

pub struct ChainCore {
    pub blocks: Vec<BlockUnit>,
}

impl ChainCore {
    pub fn create() -> Self {
        let genesis = BlockUnit {
            index: 0,
            prev_hash: "0".into(),
            time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(),
            data: "GENESIS_BLOCK".into(),
            nonce: 0,
            hash: "GENESIS".into(),
        };
        ChainCore { blocks: vec![genesis] }
    }

    pub fn latest(&self) -> &BlockUnit {
        self.blocks.last().unwrap()
    }
}
