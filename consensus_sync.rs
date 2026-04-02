use crate::block_unit::BlockUnit;
use crate::chain_verifier;

pub fn sync_chains(chains: Vec<Vec<BlockUnit>>) -> Vec<BlockUnit> {
    let mut best = chains[0].clone();
    for chain in chains {
        if chain.len() > best.len() && chain_verifier::verify_chain(&chain) {
            best = chain;
        }
    }
    best
}
