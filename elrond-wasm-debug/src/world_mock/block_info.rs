#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_timestamp: u64,
    pub block_nonce: u64,
    pub block_round: u64,
    pub block_epoch: u64,
    pub block_random_seed: Box<[u8; 48]>,
}

impl BlockInfo {
    pub fn new() -> Self {
        BlockInfo {
            block_timestamp: 0,
            block_nonce: 0,
            block_round: 0,
            block_epoch: 0,
            block_random_seed: Box::from([0u8; 48]),
        }
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self::new()
    }
}
