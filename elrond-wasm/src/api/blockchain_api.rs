use super::{ErrorApi, ManagedTypeApi};
use crate::types::{
    Address, BigUint, EsdtTokenData, ManagedAddress, ManagedByteArray, TokenIdentifier, H256,
};
use alloc::boxed::Box;

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApi: ErrorApi + ManagedTypeApi + Clone + Sized + 'static {
    fn get_caller_legacy(&self) -> Address;

    fn get_caller(&self) -> ManagedAddress<Self> {
        ManagedAddress::from_address(self.clone(), &self.get_caller_legacy())
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn get_sc_address(&self) -> ManagedAddress<Self> {
        ManagedAddress::from_address(self.clone(), &self.get_sc_address_legacy())
    }

    fn get_owner_address_legacy(&self) -> Address;

    fn get_owner_address(&self) -> ManagedAddress<Self> {
        ManagedAddress::from_address(self.clone(), &self.get_owner_address_legacy())
    }

    fn get_shard_of_address(&self, address: &Address) -> u32;

    fn is_smart_contract(&self, address: &Address) -> bool;

    fn get_balance(&self, address: &Address) -> BigUint<Self>;

    fn get_state_root_hash(&self) -> H256;

    #[inline]
    fn get_state_root_hash_managed(&self) -> ManagedByteArray<Self, 32> {
        ManagedByteArray::new_from_bytes(self.clone(), self.get_state_root_hash().as_array())
    }

    fn get_tx_hash(&self) -> H256;

    fn get_tx_hash_managed(&self) -> ManagedByteArray<Self, 32> {
        ManagedByteArray::new_from_bytes(self.clone(), self.get_tx_hash().as_array())
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn get_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_block_random_seed_managed(&self) -> ManagedByteArray<Self, 48> {
        ManagedByteArray::new_from_bytes(self.clone(), &*self.get_block_random_seed())
    }

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed(&self) -> Box<[u8; 48]>;

    fn get_prev_block_random_seed_managed(&self) -> ManagedByteArray<Self, 48> {
        ManagedByteArray::new_from_bytes(self.clone(), &*self.get_prev_block_random_seed())
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address: &Address,
        token_id: &TokenIdentifier<Self>,
    ) -> u64;

    fn get_esdt_balance(
        &self,
        address: &ManagedAddress<Self>,
        token_id: &TokenIdentifier<Self>,
        nonce: u64,
    ) -> BigUint<Self>;

    fn get_esdt_token_data(
        &self,
        address: &ManagedAddress<Self>,
        token_id: &TokenIdentifier<Self>,
        nonce: u64,
    ) -> EsdtTokenData<Self>;
}
