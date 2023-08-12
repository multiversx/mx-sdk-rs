use crate::api::{
    managed_types::managed_buffer_api_node::{
        unsafe_buffer_load_address, unsafe_buffer_load_token_identifier,
    },
    VmApiImpl,
};
use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, ManagedBufferApiImpl, RawHandle},
    types::heap::{Address, Box, H256},
};

extern "C" {
    // address utils
    fn getSCAddress(resultOffset: *mut u8);

    fn managedSCAddress(resultHandle: i32);
    fn managedOwnerAddress(resultHandle: i32);

    fn getCaller(resultOffset: *mut u8);

    fn managedCaller(resultHandle: i32);

    fn getShardOfAddress(address_ptr: *const u8) -> i32;
    fn isSmartContract(address_ptr: *const u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn getFunction(functionOffset: *const u8) -> i32;

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);
    fn getOriginalTxHash(resultOffset: *const u8);

    // Managed versions of the above
    fn managedGetPrevBlockRandomSeed(resultHandle: i32);
    fn managedGetBlockRandomSeed(resultHandle: i32);
    fn managedGetStateRootHash(resultHandle: i32);
    fn managedGetOriginalTxHash(resultHandle: i32);

    // big int API
    fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
    fn bigIntGetESDTExternalBalance(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
        dest: i32,
    );

    // ESDT NFT
    fn getCurrentESDTNFTNonce(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
    ) -> i64;

    fn managedGetESDTTokenData(
        addressHandle: i32,
        tokenIDHandle: i32,
        nonce: i64,
        valueHandle: i32,
        propertiesHandle: i32,
        hashHandle: i32,
        nameHandle: i32,
        attributesHandle: i32,
        creatorHandle: i32,
        royaltiesHandle: i32,
        urisHandle: i32,
    );

    fn managedIsESDTFrozen(addressHandle: i32, tokenIDHandle: i32, nonce: i64) -> i32;
    fn managedIsESDTPaused(tokenIDHandle: i32) -> i32;
    fn managedIsESDTLimitedTransfer(tokenIDHandle: i32) -> i32;

    fn getESDTLocalRoles(tokenhandle: i32) -> i64;
}

impl BlockchainApi for VmApiImpl {
    type BlockchainApiImpl = VmApiImpl;

    #[inline]
    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        VmApiImpl {}
    }
}

impl BlockchainApiImpl for VmApiImpl {
    #[inline]
    fn get_caller_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getCaller(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_caller_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedCaller(dest);
        }
    }

    #[inline]
    fn get_sc_address_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getSCAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedSCAddress(dest);
        }
    }

    #[inline]
    fn load_owner_address_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedOwnerAddress(dest);
        }
    }

    #[inline]
    fn get_shard_of_address_legacy(&self, address: &Address) -> u32 {
        unsafe { getShardOfAddress(address.as_ref().as_ptr()) as u32 }
    }

    #[inline]
    fn get_shard_of_address(&self, address_handle: Self::ManagedBufferHandle) -> u32 {
        unsafe { getShardOfAddress(unsafe_buffer_load_address(address_handle)) as u32 }
    }

    #[inline]
    fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
    }

    #[inline]
    fn is_smart_contract(&self, address_handle: Self::ManagedBufferHandle) -> bool {
        unsafe { isSmartContract(unsafe_buffer_load_address(address_handle)) > 0 }
    }

    #[inline]
    fn load_balance_legacy(&self, dest: Self::BigIntHandle, address: &Address) {
        unsafe {
            bigIntGetExternalBalance(address.as_ref().as_ptr(), dest);
        }
    }

    #[inline]
    fn load_balance(&self, dest: Self::BigIntHandle, address_handle: Self::ManagedBufferHandle) {
        unsafe {
            bigIntGetExternalBalance(unsafe_buffer_load_address(address_handle), dest);
        }
    }

    #[inline]
    fn load_state_root_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetStateRootHash(dest);
        }
    }

    #[inline]
    fn get_tx_hash_legacy(&self) -> H256 {
        unsafe {
            let mut res = H256::zero();
            getOriginalTxHash(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn load_tx_hash_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetOriginalTxHash(dest);
        }
    }

    #[inline]
    fn get_gas_left(&self) -> u64 {
        unsafe { getGasLeft() as u64 }
    }

    #[inline]
    fn get_block_timestamp(&self) -> u64 {
        unsafe { getBlockTimestamp() as u64 }
    }

    #[inline]
    fn get_block_nonce(&self) -> u64 {
        unsafe { getBlockNonce() as u64 }
    }

    #[inline]
    fn get_block_round(&self) -> u64 {
        unsafe { getBlockRound() as u64 }
    }

    #[inline]
    fn get_block_epoch(&self) -> u64 {
        unsafe { getBlockEpoch() as u64 }
    }

    #[inline]
    fn load_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetBlockRandomSeed(dest);
        }
    }

    #[inline]
    fn get_prev_block_timestamp(&self) -> u64 {
        unsafe { getPrevBlockTimestamp() as u64 }
    }

    #[inline]
    fn get_prev_block_nonce(&self) -> u64 {
        unsafe { getPrevBlockNonce() as u64 }
    }

    #[inline]
    fn get_prev_block_round(&self) -> u64 {
        unsafe { getPrevBlockRound() as u64 }
    }

    #[inline]
    fn get_prev_block_epoch(&self) -> u64 {
        unsafe { getPrevBlockEpoch() as u64 }
    }

    #[inline]
    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unsafe {
            let mut res = [0u8; 48];
            getPrevBlockRandomSeed(res.as_mut_ptr());
            Box::new(res)
        }
    }

    #[inline]
    fn load_prev_block_random_seed_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedGetPrevBlockRandomSeed(dest);
        }
    }

    #[inline]
    fn get_current_esdt_nft_nonce(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> u64 {
        unsafe {
            let token_identifier_len = self.mb_len(token_id_handle);
            getCurrentESDTNFTNonce(
                unsafe_buffer_load_address(address_handle),
                unsafe_buffer_load_token_identifier(token_id_handle),
                token_identifier_len as i32,
            ) as u64
        }
    }

    fn load_esdt_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    ) {
        let token_identifier_len = self.mb_len(token_id_handle);
        unsafe {
            bigIntGetESDTExternalBalance(
                unsafe_buffer_load_address(address_handle),
                unsafe_buffer_load_token_identifier(token_id_handle),
                token_identifier_len as i32,
                nonce as i64,
                dest,
            );
        }
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
        unsafe {
            managedGetESDTTokenData(
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
            );
        }
    }

    fn check_esdt_frozen(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
    ) -> bool {
        unsafe { managedIsESDTFrozen(address_handle, token_id_handle, nonce as i64) > 0 }
    }

    fn check_esdt_paused(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        unsafe { managedIsESDTPaused(token_id_handle) > 0 }
    }

    fn check_esdt_limited_transfer(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        unsafe { managedIsESDTLimitedTransfer(token_id_handle) > 0 }
    }

    fn load_esdt_local_roles(
        &self,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> multiversx_sc::types::EsdtLocalRoleFlags {
        unsafe {
            multiversx_sc::types::EsdtLocalRoleFlags::from_bits_unchecked(getESDTLocalRoles(
                token_id_handle,
            ) as u64)
        }
    }
}
