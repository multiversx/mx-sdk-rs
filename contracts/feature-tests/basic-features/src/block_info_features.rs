multiversx_sc::imports!();

/// Block info getters.
#[multiversx_sc::module]
pub trait BlockInfoFeatures {
    #[view]
    fn get_block_timestamp(&self) -> TimestampSeconds {
        self.blockchain().get_block_timestamp_seconds()
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
    fn get_prev_block_timestamp(&self) -> TimestampSeconds {
        self.blockchain().get_prev_block_timestamp_seconds()
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

    #[view]
    fn epoch_info(&self) -> MultiValue4<DurationMillis, TimestampMillis, u64, u64> {
        (
            self.blockchain().get_block_round_time_millis(),
            self.blockchain().epoch_start_block_timestamp_millis(),
            self.blockchain().epoch_start_block_nonce(),
            self.blockchain().epoch_start_block_round(),
        )
            .into()
    }

    #[view]
    fn code_hash(&self, address: ManagedAddress) -> ManagedBuffer {
        self.blockchain().get_code_hash(&address)
    }

    /// Prev block timestamp (ms, then s), current block timestamp (ms, then s)
    #[view]
    fn get_block_timestamps(
        &self,
    ) -> MultiValue4<TimestampMillis, TimestampSeconds, TimestampMillis, TimestampSeconds> {
        (
            self.blockchain().get_prev_block_timestamp_millis(),
            self.blockchain().get_prev_block_timestamp_seconds(),
            self.blockchain().get_block_timestamp_millis(),
            self.blockchain().get_block_timestamp_seconds(),
        )
            .into()
    }

    #[view]
    fn get_block_timestamp_ms(&self) -> TimestampMillis {
        self.blockchain().get_block_timestamp_millis()
    }

    #[view]
    fn get_prev_block_timestamp_ms(&self) -> TimestampMillis {
        self.blockchain().get_prev_block_timestamp_millis()
    }
}
