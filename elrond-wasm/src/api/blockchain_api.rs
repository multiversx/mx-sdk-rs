use super::{Handle, ManagedTypeApi, ManagedTypeApiImpl};
use crate::types::{
    heap::{Address, Box, H256},
    BigUint, EsdtLocalRoleFlags, EsdtTokenData, ManagedAddress, ManagedByteArray, TokenIdentifier,
};

pub trait BlockchainApi: ManagedTypeApi {
    type BlockchainApiImpl: BlockchainApiImpl;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl;
}

/// Interface to be used by the actual smart contract code.
///
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern
/// to isolate mock state mutability from the contract interface.
pub trait BlockchainApiImpl: ManagedTypeApiImpl {
    fn get_caller_legacy(&self) -> Address;

    fn get_caller_handle(&self) -> Handle {
        self.mb_new_from_bytes(self.get_caller_legacy().as_bytes())
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn get_sc_address_handle(&self) -> Handle {
        self.mb_new_from_bytes(self.get_sc_address_legacy().as_bytes())
    }

    fn get_owner_address_legacy(&self) -> Address;

    fn get_owner_address_handle(&self) -> Handle {
        self.mb_new_from_bytes(self.get_owner_address_legacy().as_bytes())
    }

    fn get_shard_of_address_legacy(&self, address: &Address) -> u32;

    fn get_shard_of_address(&self, address_handle: Handle) -> u32 {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.get_shard_of_address_legacy(&address)
    }

    fn is_smart_contract_legacy(&self, address: &Address) -> bool;

    fn is_smart_contract(&self, address_handle: Handle) -> bool {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.is_smart_contract_legacy(&address)
    }

    fn get_balance_legacy(&self, address: &Address) -> Handle;

    fn get_balance_handle(&self, address_handle: Handle) -> Handle {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.get_balance_legacy(&address)
    }

    fn get_state_root_hash_legacy(&self) -> H256;

    #[inline]
    fn get_state_root_hash<M: ManagedTypeApi>(&self) -> ManagedByteArray<M, 32> {
        ManagedByteArray::new_from_bytes(self.get_state_root_hash_legacy().as_array())
    }

    fn get_tx_hash_legacy(&self) -> H256;

    fn get_tx_hash<M: ManagedTypeApi>(&self) -> ManagedByteArray<M, 32> {
        ManagedByteArray::new_from_bytes(self.get_tx_hash_legacy().as_array())
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn get_block_random_seed<M: ManagedTypeApi>(&self) -> ManagedByteArray<M, 48> {
        ManagedByteArray::new_from_bytes(&*self.get_block_random_seed_legacy())
    }

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn get_prev_block_random_seed<M: ManagedTypeApi>(&self) -> ManagedByteArray<M, 48> {
        ManagedByteArray::new_from_bytes(&*self.get_prev_block_random_seed_legacy())
    }

    fn get_current_esdt_nft_nonce<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token_id: &TokenIdentifier<M>,
    ) -> u64;

    fn get_esdt_balance<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token_id: &TokenIdentifier<M>,
        nonce: u64,
    ) -> BigUint<M>;

    fn get_esdt_token_data<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token_id: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M>;

    fn get_esdt_local_roles<M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
    ) -> EsdtLocalRoleFlags;
}
