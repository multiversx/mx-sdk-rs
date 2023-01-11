use super::VmApiImpl;
use multiversx_sc::{
    api::{CallValueApi, CallValueApiImpl, StaticVarApiImpl},
    types::{EsdtTokenType, ManagedType, TokenIdentifier},
};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn managedGetMultiESDTCallValue(resultHandle: i32);

    fn getNumESDTTransfers() -> i32;

    // single ESDT transfer
    fn bigIntGetESDTCallValue(dest: i32);
    fn getESDTTokenName(resultOffset: *const u8) -> i32;
    fn getESDTTokenNonce() -> i64;
    fn getESDTTokenType() -> i32;

    // ESDT by index
    fn bigIntGetESDTCallValueByIndex(dest: i32, index: i32);
    fn getESDTTokenNameByIndex(resultOffset: *const u8, index: i32) -> i32;
    fn getESDTTokenNonceByIndex(index: i32) -> i64;
    fn getESDTTokenTypeByIndex(index: i32) -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_egld_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    #[cfg(not(feature = "ei-unmanaged-node"))]
    fn load_all_esdt_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiESDTCallValue(dest_handle);
        }
    }

    fn esdt_num_transfers(&self) -> usize {
        unsafe { getNumESDTTransfers() as usize }
    }

    fn load_single_esdt_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetESDTCallValue(dest);
        }
    }

    fn token(&self) -> Option<Self::ManagedBufferHandle> {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenName(name_buffer.as_mut_ptr());
            if name_len == 0 {
                None
            } else {
                Some(
                    TokenIdentifier::<Self>::from_esdt_bytes(&name_buffer[..name_len as usize])
                        .get_raw_handle(),
                )
            }
        }
    }

    fn esdt_token_nonce(&self) -> u64 {
        unsafe { getESDTTokenNonce() as u64 }
    }

    fn esdt_token_type(&self) -> EsdtTokenType {
        unsafe { (getESDTTokenType() as u8).into() }
    }

    fn esdt_value_by_index(&self, index: usize) -> Self::BigIntHandle {
        unsafe {
            let value_handle = self.next_handle();
            bigIntGetESDTCallValueByIndex(value_handle, index as i32);
            value_handle
        }
    }

    fn token_by_index(&self, index: usize) -> Self::ManagedBufferHandle {
        unsafe {
            let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
            let name_len = getESDTTokenNameByIndex(name_buffer.as_mut_ptr(), index as i32);

            TokenIdentifier::<Self>::from_esdt_bytes(&name_buffer[..name_len as usize])
                .get_raw_handle()
        }
    }

    fn esdt_token_nonce_by_index(&self, index: usize) -> u64 {
        unsafe { getESDTTokenNonceByIndex(index as i32) as u64 }
    }

    fn esdt_token_type_by_index(&self, index: usize) -> EsdtTokenType {
        unsafe { (getESDTTokenTypeByIndex(index as i32) as u8).into() }
    }
}
