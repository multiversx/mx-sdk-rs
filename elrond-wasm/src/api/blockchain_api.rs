use super::{ErrorApi, ErrorApiImpl, ManagedTypeApi, StorageReadApi, StorageReadApiImpl};
use crate::{
    err_msg,
    storage::{self, StorageKey},
    types::{
        Address, BigUint, EsdtLocalRole, EsdtLocalRoleFlags, EsdtTokenData, ManagedAddress,
        ManagedBuffer, ManagedByteArray, TokenIdentifier, H256,
    },
};
use alloc::boxed::Box;

pub trait BlockchainApi: ManagedTypeApi {
    type BlockchainApiImpl: BlockchainApiImpl<ManagedTypeApi = Self>;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl;
}

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApiImpl {
    type ManagedTypeApi: ManagedTypeApi;

    fn get_caller_legacy(&self) -> Address;

    fn get_caller(&self) -> ManagedAddress<Self::ManagedTypeApi> {
        ManagedAddress::from_address(&self.get_caller_legacy())
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn get_sc_address(&self) -> ManagedAddress<Self::ManagedTypeApi> {
        ManagedAddress::from_address(&self.get_sc_address_legacy())
    }

    fn get_owner_address_legacy(&self) -> Address;

    fn get_owner_address(&self) -> ManagedAddress<Self::ManagedTypeApi> {
        ManagedAddress::from_address(&self.get_owner_address_legacy())
    }

    fn get_shard_of_address_legacy(&self, address: &Address) -> u32;

    fn get_shard_of_address(&self, address: &ManagedAddress<Self::ManagedTypeApi>) -> u32 {
        self.get_shard_of_address_legacy(&address.to_address())
    }

    fn is_smart_contract_legacy(&self, address: &Address) -> bool;

    fn is_smart_contract(&self, address: &ManagedAddress<Self::ManagedTypeApi>) -> bool {
        self.is_smart_contract_legacy(&address.to_address())
    }

    fn get_balance_legacy(&self, address: &Address) -> BigUint<Self::ManagedTypeApi>;

    fn get_balance(
        &self,
        address: &ManagedAddress<Self::ManagedTypeApi>,
    ) -> BigUint<Self::ManagedTypeApi> {
        self.get_balance_legacy(&address.to_address())
    }

    fn get_state_root_hash_legacy(&self) -> H256;

    #[inline]
    fn get_state_root_hash(&self) -> ManagedByteArray<Self::ManagedTypeApi, 32> {
        ManagedByteArray::new_from_bytes(self.get_state_root_hash_legacy().as_array())
    }

    fn get_tx_hash_legacy(&self) -> H256;

    fn get_tx_hash(&self) -> ManagedByteArray<Self::ManagedTypeApi, 32> {
        ManagedByteArray::new_from_bytes(self.get_tx_hash_legacy().as_array())
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn get_block_random_seed(&self) -> ManagedByteArray<Self::ManagedTypeApi, 48> {
        ManagedByteArray::new_from_bytes(&*self.get_block_random_seed_legacy())
    }

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn get_prev_block_random_seed(&self) -> ManagedByteArray<Self::ManagedTypeApi, 48> {
        ManagedByteArray::new_from_bytes(&*self.get_prev_block_random_seed_legacy())
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address: &ManagedAddress<Self::ManagedTypeApi>,
        token_id: &TokenIdentifier<Self::ManagedTypeApi>,
    ) -> u64;

    fn get_esdt_balance(
        &self,
        address: &ManagedAddress<Self::ManagedTypeApi>,
        token_id: &TokenIdentifier<Self::ManagedTypeApi>,
        nonce: u64,
    ) -> BigUint<Self::ManagedTypeApi>;

    fn get_esdt_token_data(
        &self,
        address: &ManagedAddress<Self::ManagedTypeApi>,
        token_id: &TokenIdentifier<Self::ManagedTypeApi>,
        nonce: u64,
    ) -> EsdtTokenData<Self::ManagedTypeApi>;

    fn get_esdt_local_roles(
        &self,
        token_id: &TokenIdentifier<Self::ManagedTypeApi>,
    ) -> EsdtLocalRoleFlags;
}
