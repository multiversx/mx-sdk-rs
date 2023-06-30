use super::{HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl, RawHandle};
use crate::types::{
    heap::{Address, Box, H256},
    EsdtLocalRoleFlags,
};

pub trait BlockchainApi: ManagedTypeApi {
    type BlockchainApiImpl: BlockchainApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

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

    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_caller_legacy().as_bytes());
    }

    fn get_sc_address_legacy(&self) -> Address;

    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_sc_address_legacy().as_bytes())
    }

    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle);

    fn get_shard_of_address_legacy(&self, address: &Address) -> u32;

    fn get_shard_of_address(&self, address_handle: Self::ManagedBufferHandle) -> u32 {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.get_shard_of_address_legacy(&address)
    }

    fn is_smart_contract_legacy(&self, address: &Address) -> bool;

    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.is_smart_contract_legacy(&address)
    }

    fn load_balance_legacy(&self, dest: Self::BigIntHandle, address: &Address);

    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        let mut address = Address::zero();
        let _ = self.mb_load_slice(address_handle, 0, address.as_mut());
        self.load_balance_legacy(dest, &address);
    }

    fn load_state_root_hash_managed(&self, dest: Self::ManagedBufferHandle);

    fn get_tx_hash_legacy(&self) -> H256;

    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_tx_hash_legacy().as_bytes());
    }

    fn get_gas_left(&self) -> u64;

    fn get_block_timestamp(&self) -> u64;

    fn get_block_nonce(&self) -> u64;

    fn get_block_round(&self) -> u64;

    fn get_block_epoch(&self) -> u64;

    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle);

    fn get_prev_block_timestamp(&self) -> u64;

    fn get_prev_block_nonce(&self) -> u64;

    fn get_prev_block_round(&self) -> u64;

    fn get_prev_block_epoch(&self) -> u64;

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]>;

    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.mb_overwrite(dest, self.get_prev_block_random_seed_legacy().as_slice());
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> u64;

    fn load_esdt_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    );

    #[allow(clippy::too_many_arguments)]
    fn managed_get_esdt_token_data(
        &self,
        address_handle: RawHandle,
        token_id_handle: RawHandle,
        nonce: u64,
        value_handle: RawHandle,
        properties_handle: RawHandle,
        hash_handle: RawHandle,
        name_handle: RawHandle,
        attributes_handle: RawHandle,
        creator_handle: RawHandle,
        royalties_handle: RawHandle,
        uris_handle: RawHandle,
    );

    fn check_esdt_frozen(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
    ) -> bool;

    fn check_esdt_paused(&self, token_id_handle: Self::ManagedBufferHandle) -> bool;

    fn check_esdt_limited_transfer(&self, token_id_handle: Self::ManagedBufferHandle) -> bool;

    fn load_esdt_local_roles(
        &self,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> EsdtLocalRoleFlags;
}
