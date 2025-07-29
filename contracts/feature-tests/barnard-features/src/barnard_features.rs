#![no_std]

pub mod barnard_features_proxy;

use multiversx_sc::imports::*;

pub type EsdtTokenDataMultiValue<M> = MultiValue9<
    EsdtTokenType,
    BigUint<M>,
    bool,
    ManagedBuffer<M>,
    ManagedBuffer<M>,
    ManagedBuffer<M>,
    ManagedAddress<M>,
    BigUint<M>,
    ManagedVec<M, ManagedBuffer<M>>,
>;

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

    /// Prev block timestamp (ms, then s), current block timestamp (ms, then s)
    #[view]
    fn get_block_timestamps(&self) -> MultiValue4<u64, u64, u64, u64> {
        (
            self.blockchain().get_prev_block_timestamp_ms(),
            self.blockchain().get_prev_block_timestamp(),
            self.blockchain().get_block_timestamp_ms(),
            self.blockchain().get_block_timestamp(),
        )
            .into()
    }

    #[view]
    fn get_block_timestamp_ms(&self) -> u64 {
        self.blockchain().get_block_timestamp_ms()
    }

    #[view]
    fn get_prev_block_timestamp_ms(&self) -> u64 {
        self.blockchain().get_prev_block_timestamp_ms()
    }

    /// Different implementation based on feature flag.
    ///
    /// TODO: deduplicate after Barnard release.
    #[view]
    fn get_esdt_token_data(
        &self,
        address: ManagedAddress,
        token_id: TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenDataMultiValue<Self::Api> {
        let token_data = self
            .blockchain()
            .get_esdt_token_data(&address, &token_id, nonce);

        (
            token_data.token_type,
            token_data.amount,
            token_data.frozen,
            token_data.hash,
            token_data.name,
            token_data.attributes,
            token_data.creator,
            token_data.royalties,
            token_data.uris,
        )
            .into()
    }
}
