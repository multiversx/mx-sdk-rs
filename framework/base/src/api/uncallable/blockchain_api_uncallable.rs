use alloc::boxed::Box;

use crate::{
    api::{BlockchainApi, BlockchainApiImpl, RawHandle},
    types::heap::{Address, H256},
};

use super::UncallableApi;

impl BlockchainApi for UncallableApi {
    type BlockchainApiImpl = UncallableApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        unreachable!()
    }
}

impl BlockchainApiImpl for UncallableApi {
    fn get_sc_address_legacy(&self) -> Address {
        unreachable!()
    }

    fn load_owner_address_managed(&self, _dest: Self::ManagedBufferHandle) {
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

    fn load_balance_legacy(&self, _dest: Self::BigIntHandle, _address: &Address) {
        unreachable!()
    }

    fn load_state_root_hash_managed(&self, _dest: Self::ManagedBufferHandle) {
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

    fn load_block_random_seed_managed(&self, _dest: Self::ManagedBufferHandle) {
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
        _address_handle: Self::ManagedBufferHandle,
        _token_id_handle: Self::ManagedBufferHandle,
    ) -> u64 {
        unreachable!()
    }

    fn load_esdt_balance(
        &self,
        _address_handle: Self::ManagedBufferHandle,
        _token_id_handle: Self::ManagedBufferHandle,
        _nonce: u64,
        _dest: Self::BigIntHandle,
    ) {
        unreachable!()
    }

    fn managed_get_esdt_token_data(
        &self,
        _address_handle: RawHandle,
        _token_id_handle: RawHandle,
        _nonce: u64,
        _value_handle: RawHandle,
        _properties_handle: RawHandle,
        _hash_handle: RawHandle,
        _name_handle: RawHandle,
        _attributes_handle: RawHandle,
        _creator_handle: RawHandle,
        _royalties_handle: RawHandle,
        _uris_handle: RawHandle,
    ) {
        unreachable!()
    }

    fn check_esdt_frozen(
        &self,
        _address_handle: Self::ManagedBufferHandle,
        _token_id_handle: Self::ManagedBufferHandle,
        _nonce: u64,
    ) -> bool {
        unreachable!()
    }

    fn check_esdt_paused(&self, _token_id_handle: Self::ManagedBufferHandle) -> bool {
        unreachable!()
    }

    fn check_esdt_limited_transfer(&self, _token_id_handle: Self::ManagedBufferHandle) -> bool {
        unreachable!()
    }

    fn load_esdt_local_roles(
        &self,
        _token_id_handle: Self::ManagedBufferHandle,
    ) -> crate::types::EsdtLocalRoleFlags {
        unreachable!()
    }
}
