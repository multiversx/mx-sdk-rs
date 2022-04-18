use core::marker::PhantomData;

use crate::{
    api::{
        BlockchainApi, BlockchainApiImpl, ErrorApi, ErrorApiImpl, ManagedTypeApi, StaticVarApiImpl,
        StorageReadApi, StorageReadApiImpl,
    },
    storage::{self},
    types::{
        BigUint, EsdtLocalRoleFlags, EsdtTokenData, ManagedAddress, ManagedByteArray, ManagedType,
        TokenIdentifier,
    },
};

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
#[derive(Default)]
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

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_caller_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_caller_legacy()
    }

    #[inline]
    pub fn get_caller(&self) -> ManagedAddress<A> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_caller_managed(handle);
        ManagedAddress::from_raw_handle(handle)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_sc_address_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_sc_address_legacy()
    }

    #[inline]
    pub fn get_sc_address(&self) -> ManagedAddress<A> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_sc_address_managed(handle);
        ManagedAddress::from_raw_handle(handle)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_owner_address_legacy(&self) -> crate::types::Address {
        A::blockchain_api_impl().get_owner_address_legacy()
    }

    #[inline]
    pub fn get_owner_address(&self) -> ManagedAddress<A> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_owner_address_managed(handle);
        ManagedAddress::from_raw_handle(handle)
    }

    pub fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            A::error_api_impl().signal_error(b"Endpoint can only be called by owner");
        }
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_shard_of_address_legacy(&self, address: &crate::types::Address) -> u32 {
        A::blockchain_api_impl().get_shard_of_address_legacy(address)
    }

    #[inline]
    pub fn get_shard_of_address(&self, address: &ManagedAddress<A>) -> u32 {
        A::blockchain_api_impl().get_shard_of_address(address.get_raw_handle())
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn is_smart_contract_legacy(&self, address: &crate::types::Address) -> bool {
        A::blockchain_api_impl().is_smart_contract_legacy(address)
    }

    #[inline]
    pub fn is_smart_contract(&self, address: &ManagedAddress<A>) -> bool {
        A::blockchain_api_impl().is_smart_contract(address.get_raw_handle())
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_balance_legacy(&self, address: &crate::types::Address) -> BigUint<A> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_balance_legacy(handle, address);
        BigUint::from_raw_handle(handle)
    }

    #[inline]
    pub fn get_balance(&self, address: &ManagedAddress<A>) -> BigUint<A> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_balance(handle, address.get_raw_handle());
        BigUint::from_raw_handle(handle)
    }

    #[inline]
    pub fn get_sc_balance(&self, token: &TokenIdentifier<A>, nonce: u64) -> BigUint<A> {
        if token.is_egld() {
            self.get_balance(&self.get_sc_address())
        } else {
            self.get_esdt_balance(&self.get_sc_address(), token, nonce)
        }
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_state_root_hash_legacy(&self) -> crate::types::H256 {
        A::blockchain_api_impl().get_state_root_hash_legacy()
    }

    #[inline]
    pub fn get_state_root_hash(&self) -> ManagedByteArray<A, 32> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_state_root_hash_managed(handle);
        ManagedByteArray::from_raw_handle(handle)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_tx_hash_legacy(&self) -> crate::types::H256 {
        A::blockchain_api_impl().get_tx_hash_legacy()
    }

    #[inline]
    pub fn get_tx_hash(&self) -> ManagedByteArray<A, 32> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_tx_hash_managed(handle);
        ManagedByteArray::from_raw_handle(handle)
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

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        A::blockchain_api_impl().get_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_block_random_seed_managed(handle);
        ManagedByteArray::from_raw_handle(handle)
    }

    // #[inline]
    // pub fn get_block_random_seed(&self) -> ManagedByteArray<A, 48> {
    //     let handle = A::static_var_api_impl().next_handle();
    //     A::blockchain_api_impl().load_block_random_seed(handle);
    //     ManagedByteArray::from_raw_handle(handle)
    // }

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

    #[cfg(feature = "alloc")]
    #[inline]
    pub fn get_prev_block_random_seed_legacy(&self) -> crate::types::Box<[u8; 48]> {
        A::blockchain_api_impl().get_prev_block_random_seed_legacy()
    }

    #[inline]
    pub fn get_prev_block_random_seed(&self) -> ManagedByteArray<A, 48> {
        let handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_prev_block_random_seed_managed(handle);
        ManagedByteArray::from_raw_handle(handle)
    }

    #[inline]
    pub fn get_current_esdt_nft_nonce(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
    ) -> u64 {
        A::blockchain_api_impl()
            .get_current_esdt_nft_nonce(address.get_raw_handle(), token_id.get_raw_handle())
    }

    #[inline]
    pub fn get_esdt_balance(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> BigUint<A> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::blockchain_api_impl().load_esdt_balance(
            address.get_raw_handle(),
            token_id.get_raw_handle(),
            nonce,
            result_handle,
        );
        BigUint::from_raw_handle(result_handle)
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

    #[inline]
    pub fn get_esdt_local_roles(&self, token_id: &TokenIdentifier<A>) -> EsdtLocalRoleFlags {
        A::blockchain_api_impl().get_esdt_local_roles(token_id.get_raw_handle())
    }
}

impl<A> BlockchainWrapper<A>
where
    A: BlockchainApi + StorageReadApi + ManagedTypeApi + ErrorApi,
{
    /// Retrieves validator rewards, as set by the protocol.
    #[inline]
    pub fn get_cumulated_validator_rewards(&self) -> BigUint<A> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::storage_read_api_impl()
            .storage_load_big_uint_raw(storage::protected_keys::ELROND_REWARD_KEY, result_handle);
        BigUint::from_raw_handle(result_handle)
    }
}
