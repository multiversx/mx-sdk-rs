use super::{ErrorApi, ManagedTypeApi, StorageReadApi};
use crate::storage::{self, StorageKey};
use crate::types::{
    Address, BigUint, BoxedBytes, EsdtLocalRole, EsdtTokenData, ManagedAddress, ManagedBuffer,
    ManagedByteArray, ManagedType, TokenIdentifier, Vec, H256,
};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApi: ErrorApi + Clone + Sized + 'static {
    type Storage: StorageReadApi + ManagedTypeApi + 'static;

    type TypeManager: ManagedTypeApi + 'static;

    fn storage_manager(&self) -> Self::Storage;

    fn type_manager(&self) -> Self::TypeManager;

    fn get_caller_legacy(&self) -> Address;

    fn get_caller(&self) -> ManagedAddress<Self::TypeManager> {
        ManagedAddress::from_address(self.type_manager(), &self.get_caller_legacy())
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn get_sc_address(&self) -> ManagedAddress<Self::TypeManager> {
        ManagedAddress::from_address(self.type_manager(), &self.get_sc_address_legacy())
    }

    fn get_owner_address_legacy(&self) -> Address;

    fn get_owner_address(&self) -> ManagedAddress<Self::TypeManager> {
        ManagedAddress::from_address(self.type_manager(), &self.get_owner_address_legacy())
    }

    fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            self.signal_error(b"Endpoint can only be called by owner");
        }
    }

    fn get_shard_of_address(&self, address: &Address) -> u32;

    fn is_smart_contract(&self, address: &Address) -> bool;

    fn get_balance(&self, address: &Address) -> BigUint<Self::TypeManager>;

    fn get_sc_balance(
        &self,
        token: &TokenIdentifier<Self::TypeManager>,
        nonce: u64,
    ) -> BigUint<Self::TypeManager> {
        if token.is_egld() {
            self.get_balance(&self.get_sc_address_legacy())
        } else {
            self.get_esdt_balance(&self.get_sc_address(), token, nonce)
        }
    }

    fn get_state_root_hash(&self) -> H256;

    #[inline]
    fn get_state_root_hash_managed(&self) -> ManagedByteArray<Self::TypeManager, 32> {
        ManagedByteArray::new_from_bytes(self.type_manager(), self.get_state_root_hash().as_array())
    }

    fn get_tx_hash(&self) -> H256;

    fn get_tx_hash_managed(&self) -> ManagedByteArray<Self::TypeManager, 32> {
        ManagedByteArray::new_from_bytes(self.type_manager(), self.get_tx_hash().as_array())
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn get_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_block_random_seed_managed(&self) -> ManagedByteArray<Self::TypeManager, 48> {
        ManagedByteArray::new_from_bytes(self.type_manager(), &*self.get_block_random_seed())
    }

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_prev_block_random_seed_managed(&self) -> ManagedByteArray<Self::TypeManager, 48> {
        ManagedByteArray::new_from_bytes(self.type_manager(), &*self.get_prev_block_random_seed())
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address: &Address,
        token_id: &TokenIdentifier<Self::TypeManager>,
    ) -> u64;

    fn get_esdt_balance(
        &self,
        address: &ManagedAddress<Self::TypeManager>,
        token_id: &TokenIdentifier<Self::TypeManager>,
        nonce: u64,
    ) -> BigUint<Self::TypeManager>;

    fn get_esdt_token_data(
        &self,
        address: &ManagedAddress<Self::TypeManager>,
        token_id: &TokenIdentifier<Self::TypeManager>,
        nonce: u64,
    ) -> EsdtTokenData<Self::TypeManager>;

    /// Retrieves validator rewards, as set by the protocol.
    /// TODO: move to the storage API, once BigUint gets refactored
    #[inline]
    fn get_cumulated_validator_rewards(&self) -> BigUint<Self::TypeManager> {
        let raw_handle = self
            .storage_manager()
            .storage_load_big_uint_raw(storage::protected_keys::ELROND_REWARD_KEY);
        BigUint::from_raw_handle(self.type_manager(), raw_handle)
    }

    /// Retrieves local roles for the token, by reading protected storage.
    #[inline]
    fn get_esdt_local_roles(
        &self,
        token_id: &TokenIdentifier<Self::TypeManager>,
    ) -> Vec<EsdtLocalRole> {
        let mut roles = Vec::new();

        let mut key = StorageKey::new(
            self.storage_manager(),
            storage::protected_keys::ELROND_ESDT_LOCAL_ROLES_KEY,
        );
        // TODO: little hack to reconcile the fact that we declared 2 different APIs
        // theoretically unsafe
        // in practice it is always the same API
        // will be refactored out when the APIs get reorganized
        let with_changed_api =
            ManagedBuffer::from_raw_handle(self.storage_manager(), token_id.get_raw_handle());
        key.append_managed_buffer(&with_changed_api);
        let raw_storage =
            storage::storage_get::<Self::Storage, BoxedBytes>(self.storage_manager(), &key);
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
