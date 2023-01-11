use crate::api::{
    managed_types::managed_buffer_api_node::{
        unsafe_buffer_load_address, unsafe_buffer_load_token_identifier,
    },
    VmApiImpl,
};
use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, ManagedBufferApi, ManagedTypeApi},
    types::{
        heap::{Address, Box, H256},
        BigUint, EsdtTokenData, EsdtTokenType, ManagedAddress, ManagedBuffer, ManagedType,
        ManagedVec, TokenIdentifier,
    },
};

#[allow(unused)]
extern "C" {
    // address utils
    fn getSCAddress(resultOffset: *mut u8);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedSCAddress(resultHandle: i32);

    fn getOwnerAddress(resultOffset: *mut u8);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedOwnerAddress(resultHandle: i32);

    fn getCaller(resultOffset: *mut u8);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedCaller(resultHandle: i32);

    fn getShardOfAddress(address_ptr: *const u8) -> i32;
    fn isSmartContract(address_ptr: *const u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn getFunction(functionOffset: *const u8) -> i32;

    fn getGasLeft() -> i64;
    fn getBlockTimestamp() -> i64;
    fn getBlockNonce() -> i64;
    fn getBlockRound() -> i64;
    fn getBlockEpoch() -> i64;
    fn getBlockRandomSeed(resultOffset: *mut u8);
    /// Currently not used.
    #[allow(dead_code)]
    fn getStateRootHash(resultOffset: *mut u8);
    fn getPrevBlockTimestamp() -> i64;
    fn getPrevBlockNonce() -> i64;
    fn getPrevBlockRound() -> i64;
    fn getPrevBlockEpoch() -> i64;
    fn getPrevBlockRandomSeed(resultOffset: *const u8);
    fn getOriginalTxHash(resultOffset: *const u8);

    // Managed versions of the above
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetPrevBlockRandomSeed(resultHandle: i32);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetBlockRandomSeed(resultHandle: i32);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetStateRootHash(resultHandle: i32);
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetOriginalTxHash(resultHandle: i32);

    // big int API
    fn bigIntSetInt64(destination: i32, value: i64);
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
    fn getESDTTokenData(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
        valueOffset: i32,
        propertiesOffset: *const u8,
        hashOffset: *const u8,
        nameOffset: *const u8,
        attributesOffset: *const u8,
        creatorOffset: *const u8,
        royaltiesOffset: i32,
        urisOffset: *const u8,
    ) -> i32;

    // helper functions for getESDTTokenData
    fn getESDTNFTNameLength(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
    ) -> i32;
    fn getESDTNFTAttributeLength(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
    ) -> i32;
    fn getESDTNFTURILength(
        address_ptr: *const u8,
        tokenIDOffset: *const u8,
        tokenIDLen: i32,
        nonce: i64,
    ) -> i32;

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

fn esdt_is_frozen(properties_bytes: &[u8; 2]) -> bool {
    properties_bytes[0] > 0 // token is frozen if the first byte is 1
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
    #[cfg(not(feature = "ei-unmanaged-node"))]
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
    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn load_sc_address_managed(&self, dest: Self::ManagedBufferHandle) {
        unsafe {
            managedSCAddress(dest);
        }
    }

    #[inline]
    fn get_owner_address_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getOwnerAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    #[cfg(not(feature = "ei-unmanaged-node"))]
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
    fn get_state_root_hash_legacy(&self) -> H256 {
        unsafe {
            let mut res = H256::zero();
            getOriginalTxHash(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    #[cfg(not(feature = "ei-unmanaged-node"))]
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
    #[cfg(not(feature = "ei-unmanaged-node"))]
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
    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        unsafe {
            let mut res = [0u8; 48];
            getBlockRandomSeed(res.as_mut_ptr());
            Box::new(res)
        }
    }

    #[inline]
    #[cfg(not(feature = "ei-unmanaged-node"))]
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
    #[cfg(not(feature = "ei-unmanaged-node"))]
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

    fn load_esdt_token_data_unmanaged<M: ManagedTypeApi>(
        &self,
        m_address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M> {
        use multiversx_sc::{api::BigIntApi, types::heap::BoxedBytes};

        let address = m_address.to_address();
        let token_bytes = token.to_boxed_bytes();
        unsafe {
            let value_handle = self.bi_new_zero();
            let mut properties_bytes = [0u8; 2]; // always 2 bytes
            let mut hash = BoxedBytes::allocate(128);

            let name_len = getESDTNFTNameLength(
                address.as_ref().as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                nonce as i64,
            ) as usize;
            let mut name_bytes = BoxedBytes::allocate(name_len);

            let attr_len = getESDTNFTAttributeLength(
                address.as_ref().as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                nonce as i64,
            ) as usize;
            let mut attr_bytes = BoxedBytes::allocate(attr_len);

            // Current implementation of the underlying API only provides one URI
            // In the future, this might be extended to multiple URIs per token,
            // Hence the EsdtTokenData receives a Vec<BoxedBytes>
            let uris_len = getESDTNFTURILength(
                address.as_ref().as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                nonce as i64,
            ) as usize;
            let mut uri_bytes = BoxedBytes::allocate(uris_len);

            let mut creator = Address::zero();
            let royalties_handle = self.bi_new_zero();

            getESDTTokenData(
                address.as_ref().as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
                nonce as i64,
                value_handle,
                properties_bytes.as_mut_ptr(),
                hash.as_mut_ptr(),
                name_bytes.as_mut_ptr(),
                attr_bytes.as_mut_ptr(),
                creator.as_mut_ptr(),
                royalties_handle,
                uri_bytes.as_mut_ptr(),
            );

            // Fungible always have a nonce of 0, so we check nonce to figure out the type
            let nonce = getCurrentESDTNFTNonce(
                address.as_ref().as_ptr(),
                token_bytes.as_ptr(),
                token_bytes.len() as i32,
            );
            let token_type = if nonce == 0 {
                EsdtTokenType::Fungible
            } else {
                EsdtTokenType::NonFungible
            };

            let frozen = esdt_is_frozen(&properties_bytes);

            let mut uris_vec = ManagedVec::new();
            uris_vec.push(ManagedBuffer::new_from_bytes(uri_bytes.as_slice()));

            EsdtTokenData {
                token_type,
                amount: BigUint::from_raw_handle(value_handle),
                frozen,
                hash: ManagedBuffer::new_from_bytes(hash.as_slice()),
                name: ManagedBuffer::new_from_bytes(name_bytes.as_slice()),
                attributes: ManagedBuffer::new_from_bytes(attr_bytes.as_slice()),
                creator: ManagedAddress::from_address(&creator),
                royalties: BigUint::from_raw_handle(royalties_handle),
                uris: uris_vec,
            }
        }
    }

    fn load_esdt_token_data<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M> {
        use multiversx_sc::api::BigIntApi;

        let managed_token_id = token.as_managed_buffer();

        // initializing outputs
        // the current version of VM does not set/overwrite them if the token is missing,
        // which is why we need to initialize them explicitly
        let value_handle = self.bi_new_zero();
        let properties_handle = self.mb_new_empty(); // TODO: replace with const_handles::MBUF_TEMPORARY_1 after VM fix
        let hash_handle = self.mb_new_empty();
        let name_handle = self.mb_new_empty();
        let attributes_handle = self.mb_new_empty();
        let creator_handle = self.mb_new_empty();
        let royalties_handle = self.bi_new_zero();
        let uris_handle = self.mb_new_empty();

        unsafe {
            managedGetESDTTokenData(
                address.get_raw_handle(),
                managed_token_id.get_raw_handle(),
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

        let token_type = if nonce == 0 {
            EsdtTokenType::Fungible
        } else {
            EsdtTokenType::NonFungible
        };

        if self.mb_len(creator_handle) == 0 {
            self.mb_overwrite(creator_handle, &[0u8; 32][..]);
        }

        // here we trust Arwen that it always gives us a properties buffer of length 2
        let mut properties_bytes = [0u8; 2];
        let _ = self.mb_load_slice(properties_handle, 0, &mut properties_bytes[..]);
        let frozen = esdt_is_frozen(&properties_bytes);

        EsdtTokenData {
            token_type,
            amount: BigUint::from_raw_handle(value_handle),
            frozen,
            hash: ManagedBuffer::from_raw_handle(hash_handle),
            name: ManagedBuffer::from_raw_handle(name_handle),
            attributes: ManagedBuffer::from_raw_handle(attributes_handle),
            creator: ManagedAddress::from_raw_handle(creator_handle),
            royalties: BigUint::from_raw_handle(royalties_handle),
            uris: ManagedVec::from_raw_handle(uris_handle),
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
