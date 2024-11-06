use super::{TxCache, TxInput};

#[derive(Debug)]
pub struct BlockchainRng;

impl BlockchainRng {
    pub fn new(_tx_input: &TxInput, _tx_cache: &TxCache) -> Self {
        BlockchainRng
    }

    pub fn next_bytes(&mut self, _length: usize) -> Vec<u8> {
        panic!("BlockchainRng not supported for wasm builds, feature `wasm-incompatible` needs to be enabled")
    }
}
