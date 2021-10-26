use alloc::boxed::Box;

use crate::{
    api::BlockchainApi,
    types::{Address, BigUint, EsdtTokenData, ManagedAddress, TokenIdentifier, H256},
};

use super::UncallableApi;

impl BlockchainApi for UncallableApi {
    fn get_sc_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_owner_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        unreachable!()
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        unreachable!()
    }

    fn get_caller_legacy(&self) -> Address {
        unreachable!()
    }

    fn get_balance_legacy(&self, _address: &Address) -> BigUint<Self> {
        unreachable!()
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        unreachable!()
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        unreachable!()
    }

    fn get_gas_left(&self) -> u64 {
        unreachable!()
    }

    fn get_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unreachable!()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_nonce(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_round(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_epoch(&self) -> u64 {
        unreachable!()
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unreachable!()
    }

    fn get_current_esdt_nft_nonce(
        &self,
        _address: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
    ) -> u64 {
        unreachable!()
    }

    // TODO: Include nonce and create a map like: TokenId -> Nonce -> Amount
    fn get_esdt_balance(
        &self,
        _address: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
    ) -> BigUint<Self> {
        unreachable!()
    }

    fn get_esdt_token_data(
        &self,
        _address: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
    ) -> EsdtTokenData<Self> {
        unreachable!()
    }
}
