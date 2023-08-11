use rand::{Fill, Rng};
use rand_seeder::{SipHasher, SipRng};

use super::{TxCache, TxInput};

#[derive(Debug)]
pub struct BlockchainRng {
    pub rng: SipRng,
}

impl BlockchainRng {
    pub fn new(tx_input: &TxInput, tx_cache: &TxCache) -> Self {
        let mut seed = Vec::new();
        seed.extend_from_slice(
            &tx_cache
                .blockchain_ref()
                .previous_block_info
                .block_random_seed[..],
        );
        seed.extend_from_slice(
            &tx_cache
                .blockchain_ref()
                .current_block_info
                .block_random_seed[..],
        );
        seed.extend_from_slice(tx_input.tx_hash.as_bytes());

        let hasher = SipHasher::from(&seed);
        Self {
            rng: hasher.into_rng(),
        }
    }

    pub fn fill<T: Fill + ?Sized>(&mut self, dest: &mut T) {
        self.rng.fill(dest);
    }

    pub fn next_bytes(&mut self, length: usize) -> Vec<u8> {
        let mut bytes = vec![0; length];
        self.fill(&mut bytes[..]);
        bytes
    }
}
