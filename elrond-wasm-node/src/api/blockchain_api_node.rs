use crate::{
    api::managed_types::managed_buffer_api_node::{
        unsafe_buffer_load_address, unsafe_buffer_load_token_identifier,
    },
    VmApiImpl,
};
use elrond_wasm::{
    api::{BlockchainApi, BlockchainApiImpl, Handle, ManagedTypeApi},
    types::{
        Address, BigUint, Box, EsdtTokenData, EsdtTokenType, ManagedAddress, ManagedBuffer,
        ManagedType, ManagedVec, TokenIdentifier, H256,
    },
};

#[allow(unused)]
extern "C" {
    // managed buffer API
    fn mBufferNew() -> i32;

    // address utils
    fn getSCAddress(resultOffset: *mut u8);
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedSCAddress(resultHandle: i32);

    fn getOwnerAddress(resultOffset: *mut u8);
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedOwnerAddress(resultHandle: i32);

    fn getCaller(resultOffset: *mut u8);
    #[cfg(not(feature = "unmanaged-ei"))]
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetPrevBlockRandomSeed(resultHandle: i32);
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetBlockRandomSeed(resultHandle: i32);
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetStateRootHash(resultHandle: i32);
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetOriginalTxHash(resultHandle: i32);

    // big int API
    fn bigIntNew(value: i64) -> i32;
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

    fn getESDTLocalRoles(tokenhandle: i32) -> i64;

    #[cfg(not(feature = "unmanaged-ei"))]
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
    fn get_sc_address_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getSCAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_sc_address_handle(&self) -> Handle {
        unsafe {
            let handle = mBufferNew();
            managedSCAddress(handle);
            handle
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_owner_address_handle(&self) -> Handle {
        unsafe {
            let handle = mBufferNew();
            managedOwnerAddress(handle);
            handle
        }
    }

    #[inline]
    fn get_shard_of_address_legacy(&self, address: &Address) -> u32 {
        unsafe { getShardOfAddress(address.as_ref().as_ptr()) as u32 }
    }

    #[inline]
    fn get_shard_of_address(&self, address_handle: Handle) -> u32 {
        unsafe { getShardOfAddress(unsafe_buffer_load_address(address_handle)) as u32 }
    }

    #[inline]
    fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
    }

    #[inline]
    fn is_smart_contract(&self, address_handle: Handle) -> bool {
        unsafe { isSmartContract(unsafe_buffer_load_address(address_handle)) > 0 }
    }

    #[inline]
    fn get_caller_legacy(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getCaller(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_caller_handle(&self) -> Handle {
        unsafe {
            let handle = mBufferNew();
            managedCaller(handle);
            handle
        }
    }

    fn get_balance_legacy(&self, address: &Address) -> Handle {
        unsafe {
            let balance_handle = bigIntNew(0);
            bigIntGetExternalBalance(address.as_ref().as_ptr(), balance_handle);
            balance_handle
        }
    }

    fn get_balance_handle(&self, address_handle: Handle) -> Handle {
        unsafe {
            let balance_handle = bigIntNew(0);
            bigIntGetExternalBalance(unsafe_buffer_load_address(address_handle), balance_handle);
            balance_handle
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_state_root_hash<M: ManagedTypeApi>(
        &self,
    ) -> elrond_wasm::types::ManagedByteArray<M, 32> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetStateRootHash(result_handle);
            elrond_wasm::types::ManagedByteArray::from_raw_handle(result_handle)
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_tx_hash<M: ManagedTypeApi>(&self) -> elrond_wasm::types::ManagedByteArray<M, 32> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetOriginalTxHash(result_handle);
            elrond_wasm::types::ManagedByteArray::from_raw_handle(result_handle)
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_block_random_seed<M: ManagedTypeApi>(
        &self,
    ) -> elrond_wasm::types::ManagedByteArray<M, 48> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetBlockRandomSeed(result_handle);
            elrond_wasm::types::ManagedByteArray::from_raw_handle(result_handle)
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
    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_prev_block_random_seed<M: ManagedTypeApi>(
        &self,
    ) -> elrond_wasm::types::ManagedByteArray<M, 48> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetPrevBlockRandomSeed(result_handle);
            elrond_wasm::types::ManagedByteArray::from_raw_handle(result_handle)
        }
    }

    #[inline]
    fn get_current_esdt_nft_nonce<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
    ) -> u64 {
        unsafe {
            getCurrentESDTNFTNonce(
                unsafe_buffer_load_address(address.get_raw_handle()),
                unsafe_buffer_load_token_identifier(token.get_raw_handle()),
                token.len() as i32,
            ) as u64
        }
    }

    fn get_esdt_balance<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> BigUint<M> {
        unsafe {
            let balance_handle = bigIntNew(0);
            bigIntGetESDTExternalBalance(
                unsafe_buffer_load_address(address.get_raw_handle()),
                unsafe_buffer_load_token_identifier(token.get_raw_handle()),
                token.len() as i32,
                nonce as i64,
                balance_handle,
            );

            BigUint::from_raw_handle(balance_handle)
        }
    }

    #[cfg(feature = "unmanaged-ei")]
    fn get_esdt_token_data<M: ManagedTypeApi>(
        &self,
        m_address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M> {
        use elrond_wasm::types::BoxedBytes;
        let address = m_address.to_address();
        unsafe {
            let value_handle = bigIntNew(0);
            let mut properties = [0u8; 2]; // always 2 bytes
            let mut hash = BoxedBytes::allocate(128);

            let name_len = getESDTNFTNameLength(
                address.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut name_bytes = BoxedBytes::allocate(name_len);

            let attr_len = getESDTNFTAttributeLength(
                address.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut attr_bytes = BoxedBytes::allocate(attr_len);

            // Current implementation of the underlying API only provides one URI
            // In the future, this might be extended to multiple URIs per token,
            // Hence the EsdtTokenData receives a Vec<BoxedBytes>
            let uris_len = getESDTNFTURILength(
                address.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut uri_bytes = BoxedBytes::allocate(uris_len);

            let mut creator = Address::zero();
            let royalties_handle = bigIntNew(0);

            getESDTTokenData(
                address.as_ref().as_ptr(),
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
                nonce as i64,
                value_handle,
                properties.as_mut_ptr(),
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
                token.to_esdt_identifier().as_ptr(),
                token.len() as i32,
            );
            let token_type = if nonce == 0 {
                EsdtTokenType::Fungible
            } else {
                EsdtTokenType::NonFungible
            };

            // Token is frozen if properties are not 0
            let frozen = properties[0] == 0 && properties[1] == 0;

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

    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_esdt_token_data<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M> {
        let managed_token_id = token.as_managed_buffer();
        unsafe {
            let value_handle = bigIntNew(0);
            let properties_handle = mBufferNew();
            let hash_handle = mBufferNew();
            let name_handle = mBufferNew();
            let attributes_handle = mBufferNew();
            let creator_handle = mBufferNew();
            let royalties_handle = bigIntNew(0);
            let uris_handle = mBufferNew();

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

            let token_type = if nonce == 0 {
                EsdtTokenType::Fungible
            } else {
                EsdtTokenType::NonFungible
            };

            // here we trust Arwen that it always gives us a properties buffer of length 2
            let properties_buffer = ManagedBuffer::<Self>::from_raw_handle(properties_handle);
            let mut properties_bytes = [0u8; 2];
            let _ = properties_buffer.load_slice(0, &mut properties_bytes[..]);
            let frozen = properties_bytes[0] == 0 && properties_bytes[1] == 0; // token is frozen if properties are not 0

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
    }

    #[cfg(not(feature = "ei-1-1"))]
    fn get_esdt_local_roles<M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
    ) -> elrond_wasm::types::EsdtLocalRoleFlags {
        use elrond_wasm::{
            api::{ErrorApiImpl, ManagedBufferApi, StorageReadApiImpl},
            storage::StorageKey,
            types::{EsdtLocalRole, EsdtLocalRoleFlags},
        };

        let mut key =
            StorageKey::new(elrond_wasm::storage::protected_keys::ELROND_ESDT_LOCAL_ROLES_KEY);
        key.append_managed_buffer(token_id.as_managed_buffer());
        let value_handle = self.storage_load_managed_buffer_raw(key.get_raw_handle());
        let value_len = self.mb_len(value_handle);
        const DATA_MAX_LEN: usize = 300;
        if value_len > DATA_MAX_LEN {
            self.signal_error(elrond_wasm::err_msg::STORAGE_VALUE_EXCEEDS_BUFFER);
        }
        let mut data_buffer = [0u8; DATA_MAX_LEN];
        // let _ = value_mb.load_slice(0, );
        let _ = self.mb_load_slice(value_handle, 0, &mut data_buffer[..value_len]);

        let mut current_index = 0;

        let mut result = EsdtLocalRoleFlags::NONE;

        while current_index < value_len {
            // first character before each role is a \n, so we skip it
            current_index += 1;

            // next is the length of the role as string
            let role_len = data_buffer[current_index];
            current_index += 1;

            // next is role's ASCII string representation
            let end_index = current_index + role_len as usize;
            let role_name = &data_buffer[current_index..end_index];
            current_index = end_index;

            result |= EsdtLocalRole::from(role_name).to_flag();
        }

        result
    }

    #[cfg(feature = "ei-1-1")]
    fn get_esdt_local_roles<M: ManagedTypeApi>(
        &self,
        token_id: &TokenIdentifier<M>,
    ) -> elrond_wasm::types::EsdtLocalRoleFlags {
        let managed_token_id = token_id.as_managed_buffer();
        unsafe {
            elrond_wasm::types::EsdtLocalRoleFlags::from_bits_unchecked(getESDTLocalRoles(
                managed_token_id.get_raw_handle(),
            ) as u64)
        }
    }
}
