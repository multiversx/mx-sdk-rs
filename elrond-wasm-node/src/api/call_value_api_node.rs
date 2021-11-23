use super::VmApiImpl;
use elrond_wasm::{
    api::CallValueApi,
    types::{BigUint, EsdtTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn bigIntNew(value: i64) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn mBufferNew() -> i32;

    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);
    fn bigIntGetESDTCallValue(dest: i32);
    fn getESDTTokenName(resultOffset: *const u8) -> i32;
    fn getESDTTokenNonce() -> i64;
    fn getESDTTokenType() -> i32;

    // multi-transfer API
    fn getNumESDTTransfers() -> i32;
    fn bigIntGetESDTCallValueByIndex(dest: i32, index: i32);
    fn getESDTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getESDTTokenNonceByIndex(index: i32) -> i64;
    fn getESDTTokenTypeByIndex(index: i32) -> i32;
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedGetMultiESDTCallValue(resultHandle: i32);

    /// TODO: decide if it is worth using or not
    #[allow(dead_code)]
    fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn egld_value(&self) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetCallValue(value_handle);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn esdt_value(&self) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetESDTCallValue(value_handle);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn token(&self) -> TokenIdentifier<Self> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                TokenIdentifier::egld()
            } else {
                TokenIdentifier::from_esdt_bytes(&name_buffer[..name_len as usize])
            }
        }
    }

    fn esdt_token_nonce(&self) -> u64 {
        unsafe { getESDTTokenNonce() as u64 }
    }

    fn esdt_token_type(&self) -> EsdtTokenType {
        unsafe { (getESDTTokenType() as u8).into() }
    }

    fn esdt_num_transfers(&self) -> usize {
        unsafe { getNumESDTTransfers() as usize }
    }

    fn esdt_value_by_index(&self, index: usize) -> BigUint<Self> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetESDTCallValueByIndex(value_handle, index as i32);
            BigUint::from_raw_handle(value_handle)
        }
    }

    fn token_by_index(&self, index: usize) -> TokenIdentifier<Self> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);
            if name_len == 0 {
                TokenIdentifier::egld()
            } else {
                TokenIdentifier::from_esdt_bytes(&name_buffer[..name_len as usize])
            }
        }
    }

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getESDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        unsafe { (getESDTTokenTypeByIndex(index as i32) as u8).into() }
    }

    #[cfg(not(feature = "unmanaged-ei"))]
    fn get_all_esdt_transfers(
        &self,
    ) -> elrond_wasm::types::ManagedVec<Self, elrond_wasm::types::EsdtTokenPayment<Self>> {
        unsafe {
            let result_handle = mBufferNew();
            managedGetMultiESDTCallValue(result_handle);
            elrond_wasm::types::ManagedVec::from_raw_handle(result_handle)
        }
    }
}
