use crate::tx_item::TxItem;

pub struct TxMemoryPool {
    pool: Vec<TxItem>,
}

impl TxMemoryPool {
    pub fn init() -> Self {
        TxMemoryPool { pool: Vec::new() }
    }

    pub fn push(&mut self, tx: TxItem) {
        self.pool.push(tx);
    }

    pub fn drain(&mut self) -> Vec<TxItem> {
        std::mem::take(&mut self.pool)
    }
}
