use crate::{
    api::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi},
    storage::{self, StorageKey},
    types::{
        Address, BigUint, BoxedBytes, EsdtLocalRole, EsdtTokenData, ManagedAddress, ManagedBuffer,
        ManagedByteArray, ManagedType, TokenIdentifier, Vec, H256,
    },
};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub struct BlockchainHelper<A>
where
    A: BlockchainApi + StorageReadApi + ManagedTypeApi + ErrorApi,
{
    pub(crate) api: A,
}

impl<A> BlockchainHelper<A>
where
    A: BlockchainApi + StorageReadApi + ManagedTypeApi + ErrorApi,
{
    pub(crate) fn new(api: A) -> Self {
        BlockchainHelper { api }
    }

    #[inline]
    pub fn get_caller_legacy(&self) -> Address {
        self.api.get_caller_legacy()
    }

    #[inline]
    pub fn get_caller(&self) -> ManagedAddress<A> {
        self.api.get_caller()
    }

    #[inline]
    pub fn get_sc_address_legacy(&self) -> Address {
        self.api.get_sc_address_legacy()
    }

    #[inline]
    pub fn get_sc_address(&self) -> ManagedAddress<A> {
        self.api.get_sc_address()
    }

    #[inline]
    pub fn get_owner_address_legacy(&self) -> Address {
        self.api.get_owner_address_legacy()
    }

    #[inline]
    pub fn get_owner_address(&self) -> ManagedAddress<A> {
        self.api.get_owner_address()
    }

    pub fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            self.api
                .signal_error(b"Endpoint can only be called by owner");
        }
    }

    #[inline]
    pub fn get_shard_of_address(&self, address: &Address) -> u32 {
        self.api.get_shard_of_address(address)
    }

    #[inline]
    pub fn is_smart_contract(&self, address: &Address) -> bool {
        self.api.is_smart_contract(address)
    }

    #[inline]
    pub fn get_balance(&self, address: &Address) -> BigUint<A> {
        self.api.get_balance(address)
    }

    #[inline]
    pub fn get_sc_balance(&self, token: &TokenIdentifier<A>, nonce: u64) -> BigUint<A> {
        if token.is_egld() {
            self.get_balance(&self.get_sc_address_legacy())
        } else {
            self.get_esdt_balance(&self.get_sc_address(), token, nonce)
        }
    }

    #[inline]
    pub fn get_state_root_hash(&self) -> H256 {
        self.api.get_state_root_hash()
    }

    #[inline]
    pub fn get_state_root_hash_managed(&self) -> ManagedByteArray<A, 32> {
        self.api.get_state_root_hash_managed()
    }

    #[inline]
    pub fn get_tx_hash(&self) -> H256 {
        self.api.get_tx_hash()
    }

    #[inline]
    pub fn get_tx_hash_managed(&self) -> ManagedByteArray<A, 32> {
        self.api.get_tx_hash_managed()
    }

    #[inline]
    pub fn get_gas_left(&self) -> u64 {
        self.api.get_gas_left()
    }

    #[inline]
    pub fn get_block_timestamp(&self) -> u64 {
        self.api.get_block_timestamp()
    }

    #[inline]
    pub fn get_block_nonce(&self) -> u64 {
        self.api.get_block_nonce()
    }

    #[inline]
    pub fn get_block_round(&self) -> u64 {
        self.api.get_block_round()
    }

    #[inline]
    pub fn get_block_epoch(&self) -> u64 {
        self.api.get_block_epoch()
    }

    #[inline]
    pub fn get_block_random_seed(&self) -> Box<[u8; 48]> {
        self.api.get_block_random_seed()
    }

    #[inline]
    pub fn get_block_random_seed_managed(&self) -> ManagedByteArray<A, 48> {
        self.api.get_block_random_seed_managed()
    }

    #[inline]
    pub fn get_prev_block_timestamp(&self) -> u64 {
        self.api.get_prev_block_timestamp()
    }

    #[inline]
    pub fn get_prev_block_nonce(&self) -> u64 {
        self.api.get_prev_block_nonce()
    }

    #[inline]
    pub fn get_prev_block_round(&self) -> u64 {
        self.api.get_prev_block_round()
    }

    #[inline]
    pub fn get_prev_block_epoch(&self) -> u64 {
        self.api.get_prev_block_epoch()
    }

    #[inline]
    pub fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
        self.api.get_prev_block_random_seed()
    }

    #[inline]
    pub fn get_prev_block_random_seed_managed(&self) -> ManagedByteArray<A, 48> {
        self.api.get_prev_block_random_seed_managed()
    }

    #[inline]
    pub fn get_current_esdt_nft_nonce(
        &self,
        address: &Address,
        token_id: &TokenIdentifier<A>,
    ) -> u64 {
        self.api.get_current_esdt_nft_nonce(address, token_id)
    }

    #[inline]
    pub fn get_esdt_balance(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> BigUint<A> {
        self.api.get_esdt_balance(address, token_id, nonce)
    }

    #[inline]
    pub fn get_esdt_token_data(
        &self,
        address: &ManagedAddress<A>,
        token_id: &TokenIdentifier<A>,
        nonce: u64,
    ) -> EsdtTokenData<A> {
        self.api.get_esdt_token_data(address, token_id, nonce)
    }

    /// Retrieves validator rewards, as set by the protocol.
    /// TODO: move to the storage API, once BigUint gets refactored
    #[inline]
    pub fn get_cumulated_validator_rewards(&self) -> BigUint<A> {
        let raw_handle = self
            .api
            .storage_load_big_uint_raw(storage::protected_keys::ELROND_REWARD_KEY);
        BigUint::from_raw_handle(self.api.clone(), raw_handle)
    }

    /// Retrieves local roles for the token, by reading protected storage.
    pub fn get_esdt_local_roles(&self, token_id: &TokenIdentifier<A>) -> Vec<EsdtLocalRole> {
        let mut roles = Vec::new();

        let mut key = StorageKey::new(
            self.api.clone(),
            storage::protected_keys::ELROND_ESDT_LOCAL_ROLES_KEY,
        );
        // TODO: little hack to reconcile the fact that we declared 2 different APIs
        // theoretically unsafe
        // in practice it is always the same API
        // will be refactored out when the APIs get reorganized
        let with_changed_api =
            ManagedBuffer::from_raw_handle(self.api.clone(), token_id.get_raw_handle());
        key.append_managed_buffer(&with_changed_api);
        let raw_storage = storage::storage_get::<A, BoxedBytes>(self.api.clone(), &key);
        let raw_storage_bytes = raw_storage.as_slice();
        let mut current_index = 0;

        while current_index < raw_storage_bytes.len() {
            // first character before each role is a \n, so we skip it
            current_index += 1;

            // next is the length of the role as string
            let role_len = raw_storage_bytes[current_index];
            current_index += 1;

            // next is role's ASCII string representation
            let end_index = current_index + role_len as usize;
            let role_name = &raw_storage_bytes[current_index..end_index];
            current_index = end_index;

            let esdt_local_role = EsdtLocalRole::from(role_name);
            roles.push(esdt_local_role);
        }

        roles
    }
}
