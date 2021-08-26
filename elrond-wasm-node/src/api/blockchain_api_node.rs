use elrond_wasm::api::BlockchainApi;
use elrond_wasm::types::{
    Address, BigUint, Box, BoxedBytes, EsdtTokenData, EsdtTokenType, TokenIdentifier, H256,
};

extern "C" {
    fn getSCAddress(resultOffset: *mut u8);
    fn getOwnerAddress(resultOffset: *mut u8);
    fn getShardOfAddress(address_ptr: *const u8) -> i32;
    fn isSmartContract(address_ptr: *const u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;

    /// Currently not used.
    #[allow(dead_code)]
    fn getFunction(functionOffset: *const u8) -> i32;

    fn getCaller(resultOffset: *mut u8);

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
}

impl BlockchainApi for crate::ArwenApiImpl {
    type Storage = Self;
    type TypeManager = Self;

    #[inline]
    fn storage_manager(&self) -> Self::Storage {
        self.clone()
    }

    #[inline]
    fn type_manager(&self) -> Self::TypeManager {
        self.clone()
    }

    #[inline]
    fn get_sc_address(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getSCAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn get_owner_address(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getOwnerAddress(res.as_mut_ptr());
            res
        }
    }

    #[inline]
    fn get_shard_of_address(&self, address: &Address) -> u32 {
        unsafe { getShardOfAddress(address.as_ref().as_ptr()) as u32 }
    }

    #[inline]
    fn is_smart_contract(&self, address: &Address) -> bool {
        unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
    }

    #[inline]
    fn get_caller(&self) -> Address {
        unsafe {
            let mut res = Address::zero();
            getCaller(res.as_mut_ptr());
            res
        }
    }

    fn get_balance(&self, address: &Address) -> BigUint<Self::Storage> {
        unsafe {
            let balance_handle = bigIntNew(0);
            bigIntGetExternalBalance(address.as_ref().as_ptr(), balance_handle);
            BigUint::from_raw_handle(self.storage_manager(), balance_handle)
        }
    }

    #[inline]
    fn get_tx_hash(&self) -> H256 {
        unsafe {
            let mut res = H256::zero();
            getOriginalTxHash(res.as_mut_ptr());
            res
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
    fn get_block_random_seed(&self) -> Box<[u8; 48]> {
        unsafe {
            let mut res = [0u8; 48];
            getBlockRandomSeed(res.as_mut_ptr());
            Box::new(res)
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
    fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
        unsafe {
            let mut res = [0u8; 48];
            getPrevBlockRandomSeed(res.as_mut_ptr());
            Box::new(res)
        }
    }

    #[inline]
    fn get_current_esdt_nft_nonce(&self, address: &Address, token: &TokenIdentifier) -> u64 {
        unsafe {
            getCurrentESDTNFTNonce(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
            ) as u64
        }
    }

    #[inline]
    fn get_esdt_balance(
        &self,
        address: &Address,
        token: &TokenIdentifier,
        nonce: u64,
    ) -> BigUint<Self::Storage> {
        unsafe {
            let balance_handle = bigIntNew(0);
            bigIntGetESDTExternalBalance(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
                nonce as i64,
                balance_handle,
            );

            BigUint::from_raw_handle(self.storage_manager(), balance_handle)
        }
    }

    #[inline]
    fn get_esdt_token_data(
        &self,
        address: &Address,
        token: &TokenIdentifier,
        nonce: u64,
    ) -> EsdtTokenData<Self::Storage> {
        unsafe {
            let value_handle = bigIntNew(0);
            let mut properties = [0u8; 2]; // always 2 bytes
            let mut hash = BoxedBytes::allocate(128);

            let name_len = getESDTNFTNameLength(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut name_buffer = BoxedBytes::allocate(name_len);

            let attr_len = getESDTNFTAttributeLength(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut attr_buffer = BoxedBytes::allocate(attr_len);

            // Current implementation of the underlying API only provides one URI
            // In the future, this might be extended to multiple URIs per token,
            // Hence the EsdtTokenData receives a Vec<BoxedBytes>
            let uris_len = getESDTNFTURILength(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
                nonce as i64,
            ) as usize;
            let mut uris_buffer = BoxedBytes::allocate(uris_len);

            let mut creator = Address::zero();
            let royalties_handle = bigIntNew(0);

            getESDTTokenData(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
                nonce as i64,
                value_handle,
                properties.as_mut_ptr(),
                hash.as_mut_ptr(),
                name_buffer.as_mut_ptr(),
                attr_buffer.as_mut_ptr(),
                creator.as_mut_ptr(),
                royalties_handle,
                uris_buffer.as_mut_ptr(),
            );

            // Fungible always have a nonce of 0, so we check nonce to figure out the type
            let nonce = getCurrentESDTNFTNonce(
                address.as_ref().as_ptr(),
                token.as_ptr(),
                token.len() as i32,
            );
            let token_type = if nonce == 0 {
                EsdtTokenType::Fungible
            } else {
                EsdtTokenType::NonFungible
            };

            // Token is frozen is properties is not 0
            let frozen = properties[0] == 0 && properties[1] == 0;

            EsdtTokenData {
                token_type,
                amount: BigUint::from_raw_handle(self.storage_manager(), value_handle),
                frozen,
                hash,
                name: name_buffer,
                attributes: attr_buffer,
                creator,
                royalties: BigUint::from_raw_handle(self.storage_manager(), royalties_handle),
                uris: [uris_buffer].to_vec(),
            }
        }
    }
}
