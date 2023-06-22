use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, HandleConstraints, ManagedBufferApiImpl, RawHandle},
    types::{Address, EsdtLocalRoleFlags, H256},
};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> BlockchainApi for VMHooksApi<VHB> {
    type BlockchainApiImpl = Self;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> BlockchainApiImpl for VMHooksApi<VHB> {
    fn get_caller_legacy(&self) -> Address {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_caller(dest.get_raw_handle_unchecked()));
    }

    fn get_sc_address_legacy(&self) -> Address {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_sc_address(dest.get_raw_handle_unchecked()));
    }

    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle) {
        self.with_vm_hooks(|vh| vh.managed_owner_address(dest.get_raw_handle_unchecked()));
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn get_shard_of_address(&self, address_handle: Self::ManagedBufferHandle) -> u32 {
        self.assert_live_handle(&address_handle);
        self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_vm_hooks(|vh| vh.get_shard_of_address(address_ptr))
        }) as u32
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        self.assert_live_handle(&address_handle);
        let result = self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_vm_hooks(|vh| vh.is_smart_contract(address_ptr))
        });
        i32_to_bool(result)
    }

    fn load_balance_legacy(&self, _dest: Self::BigIntHandle, _address: &Address) {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.assert_live_handle(&address_handle);
        self.with_temp_address_ptr(address_handle, |address_ptr: isize| {
            self.with_vm_hooks(|vh| {
                vh.big_int_get_external_balance(address_ptr, dest.get_raw_handle_unchecked())
            })
        });
    }

    fn load_state_root_hash_managed(&self, _dest: Self::ManagedBufferHandle) {
        panic!("state root hash not implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_get_original_tx_hash(dest.get_raw_handle_unchecked()));
    }

    fn get_gas_left(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_gas_left()) as u64
    }

    fn get_block_timestamp(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_timestamp()) as u64
    }

    fn get_block_nonce(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_nonce()) as u64
    }

    fn get_block_round(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_round()) as u64
    }

    fn get_block_epoch(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_block_epoch()) as u64
    }

    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| vh.managed_get_block_random_seed(dest.get_raw_handle_unchecked()));
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_timestamp()) as u64
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_nonce()) as u64
    }

    fn get_prev_block_round(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_round()) as u64
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.with_vm_hooks(|vh| vh.get_prev_block_epoch()) as u64
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        panic!("legacy BlockchainApi functionality no longer supported")
    }

    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        self.assert_live_handle(&dest);
        self.with_vm_hooks(|vh| {
            vh.managed_get_prev_block_random_seed(dest.get_raw_handle_unchecked())
        });
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> u64 {
        self.assert_live_handle(&address_handle);
        self.assert_live_handle(&token_id_handle);
        let token_id_len = self.mb_len(token_id_handle.clone());
        let result = self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_temp_buffer_ptr(token_id_handle, token_id_len, |token_id_ptr| {
                self.with_vm_hooks(|vh| {
                    vh.get_current_esdt_nft_nonce(address_ptr, token_id_ptr, token_id_len as isize)
                })
            })
        });
        result as u64
    }

    fn load_esdt_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    ) {
        self.assert_live_handle(&address_handle);
        self.assert_live_handle(&token_id_handle);
        let token_id_len = self.mb_len(token_id_handle.clone());
        self.with_temp_address_ptr(address_handle, |address_ptr| {
            self.with_temp_buffer_ptr(token_id_handle, token_id_len, |token_id_ptr| {
                self.with_vm_hooks(|vh| {
                    vh.big_int_get_esdt_external_balance(
                        address_ptr,
                        token_id_ptr,
                        token_id_len as isize,
                        nonce as i64,
                        dest.get_raw_handle_unchecked(),
                    );
                })
            })
        });
    }

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
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_get_esdt_token_data(
                address_handle,
                token_id_handle,
                nonce as i64,
                value_handle,
                properties_handle,
                hash_handle,
                name_handle,
                attributes_handle,
                creator_handle,
                royalties_handle,
                uris_handle,
            )
        });
    }

    fn check_esdt_frozen(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
    ) -> bool {
        self.assert_live_handle(&address_handle);
        self.assert_live_handle(&token_id_handle);
        let result = self.with_vm_hooks(|vh| {
            vh.managed_is_esdt_frozen(
                address_handle.get_raw_handle_unchecked(),
                token_id_handle.get_raw_handle_unchecked(),
                nonce as i64,
            )
        });
        i32_to_bool(result)
    }

    fn check_esdt_paused(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        self.assert_live_handle(&token_id_handle);
        let result = self.with_vm_hooks(|vh| {
            vh.managed_is_esdt_paused(token_id_handle.get_raw_handle_unchecked())
        });
        i32_to_bool(result)
    }

    fn check_esdt_limited_transfer(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        self.assert_live_handle(&token_id_handle);
        let result = self.with_vm_hooks(|vh| {
            vh.managed_is_esdt_limited_transfer(token_id_handle.get_raw_handle_unchecked())
        });
        i32_to_bool(result)
    }

    fn load_esdt_local_roles(
        &self,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> EsdtLocalRoleFlags {
        self.assert_live_handle(&token_id_handle);
        let result = self.with_vm_hooks(|vh| {
            vh.get_esdt_local_roles(token_id_handle.get_raw_handle_unchecked())
        });
        unsafe { EsdtLocalRoleFlags::from_bits_unchecked(result as u64) }
    }
}
