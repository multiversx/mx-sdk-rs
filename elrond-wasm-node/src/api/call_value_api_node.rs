use crate::ArwenApiImpl;
use elrond_wasm::api::CallValueApi;
use elrond_wasm::types::{BigUint, BoxedBytes, EsdtTokenType, TokenIdentifier};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn checkNoPayment();

    fn bigIntNew(value: i64) -> i32;

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

    /// TODO: decide if it is worth using or not
    #[allow(dead_code)]
    fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for ArwenApiImpl {
    type TypeManager = Self;

    #[inline]
    fn type_manager(&self) -> Self::TypeManager {
        self.clone()
    }

    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn egld_value(&self) -> BigUint<Self::TypeManager> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetCallValue(value_handle);
            BigUint::from_raw_handle(self.type_manager(), value_handle)
        }
    }

    fn esdt_value(&self) -> BigUint<Self::TypeManager> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetESDTCallValue(value_handle);
            BigUint::from_raw_handle(self.type_manager(), value_handle)
        }
    }

    fn token(&self) -> TokenIdentifier {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                TokenIdentifier::egld()
            } else {
                BoxedBytes::from(&name_buffer[..name_len as usize]).into()
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

    fn esdt_value_by_index(&self, index: usize) -> BigUint<Self::TypeManager> {
        unsafe {
            let value_handle = bigIntNew(0);
            bigIntGetESDTCallValueByIndex(value_handle, index as i32);
            BigUint::from_raw_handle(self.type_manager(), value_handle)
        }
    }

    fn token_by_index(&self, index: usize) -> TokenIdentifier {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);
            if name_len == 0 {
                TokenIdentifier::egld()
            } else {
                BoxedBytes::from(&name_buffer[..name_len as usize]).into()
            }
        }
    }

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getESDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        unsafe { (getESDTTokenTypeByIndex(index as i32) as u8).into() }
    }
}
