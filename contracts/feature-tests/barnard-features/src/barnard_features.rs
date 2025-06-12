#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait BarnardFeatures {
    #[init]
    fn init(&self) {}

    #[view(epochInfo)]
    fn epoch_info(&self) -> MultiValue4<u64, u64, u64, u64> {
        (
            self.blockchain().get_block_round_time_ms(),
            self.blockchain().epoch_start_block_timestamp_ms(),
            self.blockchain().epoch_start_block_nonce(),
            self.blockchain().epoch_start_block_round(),
        )
            .into()
    }

    #[view(codeHash)]
    fn code_hash(&self, address: ManagedAddress) -> ManagedBuffer {
        self.blockchain().get_code_hash(&address)
    }

    #[view]
    fn get_block_timestamp_ms(&self) -> u64 {
        self.blockchain().get_block_timestamp_ms()
    }

    #[view]
    fn get_prev_block_timestamp_ms(&self) -> u64 {
        self.blockchain().get_prev_block_timestamp_ms()
    }
}
