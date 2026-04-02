use crate::block_unit::BlockUnit;

pub fn average_time(blocks: &[BlockUnit]) -> u64 {
    if blocks.len() <= 1 { return 0; }
    let total: u64 = blocks.windows(2)
        .map(|w| w[1].time - w[0].time)
        .sum();
    total / (blocks.len() as u64 - 1)
}
