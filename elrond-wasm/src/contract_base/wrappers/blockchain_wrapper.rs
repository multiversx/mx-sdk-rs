use core::marker::PhantomData;

use crate::{
    api::{
        BlockchainApi, BlockchainApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi, StorageReadApi,
        StorageReadApiImpl,
    },
    storage::{self},
    types::{
        Address, BigUint, EsdtLocalRoleFlags, EsdtTokenData, ManagedAddress, ManagedByteArray,
        ManagedType, TokenIdentifier, H256,
    },
};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub struct BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<A>,
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + ManagedTypeApi + ErrorApi,
{
    pub fn new() -> Self {
        BlockchainWrapper {
            _phantom: PhantomData,
        }
    }

    #[inline]
    pub fn get_caller_legacy(&self) -> Address {
        A::blockchain_api_impl().get_caller_legacy()
    }

    #[inline]
    pub fn get_caller(&self) -> ManagedAddress<A> {
        ManagedAddress::from_raw_handle(A::blockchain_api_impl().get_caller_handle())
    }

    #[inline]
    pub fn get_sc_address_legacy(&self) -> Address {
        A::blockchain_api_impl().get_sc_address_legacy()
    }

    #[inline]
    pub fn get_sc_address(&self) -> ManagedAddress<A> {
        ManagedAddress::from_raw_handle(A::blockchain_api_impl().get_sc_address_handle())
    }

    #[inline]
    pub fn get_owner_address_legacy(&self) -> Address {
        A::blockchain_api_impl().get_owner_address_legacy()
    }

    #[inline]
    pub fn get_owner_address(&self) -> ManagedAddress<A> {
        ManagedAddress::from_raw_handle(A::blockchain_api_impl().get_owner_address_handle())
    }

    pub fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            A::error_api_impl().signal_error(b"Endpoint can only be called by owner");
        }
    }

    #[inline]
    pub fn get_shard_of_address_legacy(&self, address: &Address) -> u32 {
        A::blockchain_api_impl().get_shard_of_address_legacy(address)
    }

    #[inline]
    pub fn get_shard_of_address(&self, address: &ManagedAddress<A>) -> u32 {
        A::blockchain_api_impl().get_shard_of_address(address.get_raw_handle())
    }

    #[inline]
    pub fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        A::blockchain_api_impl().is_smart_contract_legacy(address)
    }

    #[inline]
    pub fn is_smart_contract(&self, address: &ManagedAddress<A>) -> bool {
        A::blockchain_api_impl().is_smart_contract(address.get_raw_handle())
    }

    #[inline]
    pub fn get_balance_legacy(&self, address: &Address) -> BigUint<A> {
        BigUint::from_raw_handle(A::blockchain_api_impl().get_balance_legacy(address))
    }

    #[inline]
    pub fn get_balance(&self, address: &ManagedAddress<A>) -> BigUint<A> {
        BigUint::from_raw_handle(
            A::blockchain_api_impl().get_balance_handle(address.get_raw_handle()),
        )
    }

    #[inline]
    pub fn get_sc_balance(&self, token: &TokenIdentifier<A>, nonce: u64) -> BigUint<A> {
        if token.is_egld() {
            self.get_balance(&self.get_sc_address())
        } else {
            self.get_esdt_balance(&self.get_sc_address(), token, nonce)
        }
    }

    #[inline]
    pub fn get_state_root_hash_legacy(&self) -> H256 {
        A::blockchain_api_impl().get_state_root_hash_legacy()
    }

    #[inline]
    pub fn get_state_root_hash(&self) -> ManagedByteArray<A, 32> {
        A::blockchain_api_impl().get_state_root_hash()
    }

    #[inline]
    pub fn get_tx_hash_legacy(&self) -> H256 {
        A::blockchain_api_impl().get_tx_hash_legacy()
    }

    #[inline]
    pub fn get_tx_hash(&self) -> ManagedByteArray<A, 32> {
        A::blockchain_api_impl().get_tx_hash::<A>()
    }

    #[inline]
    pub fn get_gas_left(&self) -> u64 {
        A::blockchain_api_impl().get_gas_left()
    }

    #[inline]
    pub fn get_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_block_timestamp()
    }

    #[inline]
    pub fn get_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_block_nonce()
    }

    #[inline]
    pub fn get_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_block_round()
    }

    #[inline]
    pub fn get_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_block_epoch()
    }

    #[inline]
    pub fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        A::blockchain_api_impl().get_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        A::blockchain_api_impl().get_block_random_seed::<A>()
    }

    #[inline]
    pub fn get_prev_block_timestamp(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_timestamp()
    }

    #[inline]
    pub fn get_prev_block_nonce(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_nonce()
    }

    #[inline]
    pub fn get_prev_block_round(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_round()
    }

    #[inline]
    pub fn get_prev_block_epoch(&self) -> u64 {
        A::blockchain_api_impl().get_prev_block_epoch()
    }

    #[inline]
    pub fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        A::blockchain_api_impl().get_prev_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_prev_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        A::blockchain_api_impl().get_prev_block_random_seed::<A>()
    }

    #[inline]
    pub fn get_current_esdt_nft_nonce(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
    ) -> u64 {
        A::blockchain_api_impl().get_current_esdt_nft_nonce::<A>(address, token_id)
    }

    #[inline]
    pub fn get_esdt_balance(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> BigUint<A> {
        A::blockchain_api_impl().get_esdt_balance::<A>(address, token_id, nonce)
    }

    #[inline]
    pub fn get_esdt_token_data(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> EsdtTokenData<A> {
        A::blockchain_api_impl().get_esdt_token_data::<A>(address, token_id, nonce)
    }

    pub fn get_esdt_local_roles(&self, token_id: &TokenIdentifier<A>) -> EsdtLocalRoleFlags {
        A::blockchain_api_impl().get_esdt_local_roles::<A>(token_id)
    }
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + StorageReadApi + ManagedTypeApi + ErrorApi,
{
    /// Retrieves validator rewards, as set by the protocol.
    #[inline]
    pub fn get_cumulated_validator_rewards(&self) -> BigUint<A> {
        let raw_handle = A::storage_read_api_impl()
            .storage_load_big_uint_raw(storage::protected_keys::ELROND_REWARD_KEY);
        BigUint::from_raw_handle(raw_handle)
    }
}
