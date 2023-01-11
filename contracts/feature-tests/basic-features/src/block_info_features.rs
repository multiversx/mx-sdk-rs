multiversx_sc::imports!();

/// Block info getters.
#[multiversx_sc::module]
pub trait BlockInfoFeatures {
    #[view]
    fn get_block_timestamp(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    #[view]
    fn get_block_nonce(&self) -> u64 {
        self.blockchain().get_block_nonce()
    }

    #[view]
    fn get_block_round(&self) -> u64 {
        self.blockchain().get_block_round()
    }

    #[view]
    fn get_block_epoch(&self) -> u64 {
        self.blockchain().get_block_epoch()
    }

    #[view]
    fn get_block_random_seed(&self) -> ManagedByteArray<Self::Api, 48> {
        self.blockchain().get_block_random_seed()
    }

    #[view]
    fn get_prev_block_timestamp(&self) -> u64 {
        self.blockchain().get_prev_block_timestamp()
    }

    #[view]
    fn get_prev_block_nonce(&self) -> u64 {
        self.blockchain().get_prev_block_nonce()
    }

    #[view]
    fn get_prev_block_round(&self) -> u64 {
        self.blockchain().get_prev_block_round()
    }

    #[view]
    fn get_prev_block_epoch(&self) -> u64 {
        self.blockchain().get_prev_block_epoch()
    }

    #[view]
    fn get_prev_block_random_seed(&self) -> ManagedByteArray<Self::Api, 48> {
        self.blockchain().get_prev_block_random_seed()
    }
}
