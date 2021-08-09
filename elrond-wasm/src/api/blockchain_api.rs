use super::{BigUintApi, ErrorApi, StorageReadApi};
use crate::storage;
use crate::types::{Address, BoxedBytes, EsdtLocalRole, EsdtTokenData, TokenIdentifier, Vec, H256};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApi: StorageReadApi + ErrorApi + Clone + Sized + 'static {
    /// The type of the token balances.
    /// Not named `BigUint` to avoid name collisions in types that implement multiple API traits.
    type BalanceType: BigUintApi + 'static;

    fn get_sc_address(&self) -> Address;

    fn get_owner_address(&self) -> Address;

    fn check_caller_is_owner(&self) {
        if self.get_owner_address() != self.get_caller() {
            self.signal_error(b"Endpoint can only be called by owner");
        }
    }

    fn get_shard_of_address(&self, address: &Address) -> u32;

    fn is_smart_contract(&self, address: &Address) -> bool;

    fn get_caller(&self) -> Address;

    fn get_balance(&self, address: &Address) -> Self::BalanceType;

    fn get_sc_balance(&self, token: &TokenIdentifier, nonce: u64) -> Self::BalanceType {
        let sc_address = self.get_sc_address();

        if token.is_egld() {
            self.get_balance(&sc_address)
        } else {
            self.get_esdt_balance(&sc_address, token, nonce)
        }
    }

    fn get_tx_hash(&self) -> H256;

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn get_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_current_esdt_nft_nonce(&self, address: &Address, token_id: &TokenIdentifier) -> u64;

    fn get_esdt_balance(
        &self,
        address: &Address,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> Self::BalanceType;

    fn get_esdt_token_data(
        &self,
        address: &Address,
        token_id: &TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenData<Self::BalanceType>;

    /// Retrieves validator rewards, as set by the protocol.
    /// TODO: move to the storage API, once BigUint gets refactored
    #[inline]
    fn get_cumulated_validator_rewards(&self) -> Self::BalanceType {
        storage::storage_get(self.clone(), storage::protected_keys::ELROND_REWARD_KEY)
    }

    /// Retrieves local roles for the token, by reading protected storage.
    #[inline]
    fn get_esdt_local_roles(&self, token_id: &TokenIdentifier) -> Vec<EsdtLocalRole> {
        let mut roles = Vec::new();

        let key = [
            storage::protected_keys::ELROND_ESDT_LOCAL_ROLES_KEY,
            token_id.as_esdt_identifier(),
        ]
        .concat();
        let raw_storage = storage::storage_get::<Self, BoxedBytes>(self.clone(), &key);
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
