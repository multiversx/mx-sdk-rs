pub const DEFAULT_BLOCK_ROUND_TIME_MS: u64 = 6000;

#[derive(Clone, Debug)]
pub struct BlockConfig {
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub epoch_start_block_info: BlockInfo,
    pub block_round_time_ms: u64,
}

impl Default for BlockConfig {
    fn default() -> Self {
        BlockConfig {
            previous_block_info: Default::default(),
            current_block_info: Default::default(),
            epoch_start_block_info: Default::default(),
            block_round_time_ms: DEFAULT_BLOCK_ROUND_TIME_MS,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_timestamp_ms: u64,
    pub block_nonce: u64,
    pub block_round: u64,
    pub block_epoch: u64,
    pub block_random_seed: Box<[u8; 48]>,
}

impl BlockInfo {
    pub fn new() -> Self {
        BlockInfo {
            block_timestamp_ms: 0,
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
