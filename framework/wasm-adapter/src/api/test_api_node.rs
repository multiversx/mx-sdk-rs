use super::VmApiImpl;
use multiversx_sc::api::{TestApi, TestApiImpl};

extern "C" {

    fn createAccount(addressHandle: i32, nonce: i64, balanceHandle: i32);

    fn registerNewAddress(ownerHandle: i32, nonce: i64, newAddressHandle: i32);

    fn deployContract(
        ownerHandle: i32,
        gasLimit: i64,
        valueHandle: i32,
        codePathHandle: i32,
        argumentsHandle: i32,
        resultAddressHandle: i32,
    );

    fn setStorage(addressHandle: i32, keyHandle: i32, valueHandle: i32);

    fn getStorage(addressHandle: i32, keyHandle: i32, dstHandle: i32);

    fn assumeBool(p: bool);
    fn assertBool(p: bool);

    fn startPrank(addressHandle: i32);
    fn stopPrank();

    fn setBlockTimestamp(timestamp: i64);

    fn setExternalBalance(addressHandle: i32, valueHandle: i32);

    fn setESDTExternalBalance(addressHandle: i32, tokenIdHandle: i32, valueHandle: i32);
}

impl TestApi for VmApiImpl {
    type TestApiImpl = VmApiImpl;

    #[inline]
    fn test_api_impl() -> Self::TestApiImpl {
        VmApiImpl {}
    }
}

impl TestApiImpl for VmApiImpl {
    fn create_account(
        &self,
        address: Self::ManagedBufferHandle,
        nonce: u64,
        balance: Self::BigIntHandle,
    ) {
        unsafe {
            createAccount(address, nonce as i64, balance);
        }
    }

    fn register_new_address(
        &self,
        owner: Self::ManagedBufferHandle,
        nonce: u64,
        new_address: Self::ManagedBufferHandle,
    ) {
        unsafe {
            registerNewAddress(owner, nonce as i64, new_address);
        }
    }

    // Deploy a contract whose code was previously fetched using "fetchWasmSource" in Mandos.
    fn deploy_contract(
        &self,
        owner: Self::ManagedBufferHandle,
        gas_limit: u64,
        value: Self::BigIntHandle,
        code_path: Self::ManagedBufferHandle,
        arguments: Self::ManagedBufferHandle,
        result_address_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            // let mut dest = ManagedAddress::zero();

            deployContract(
                owner,
                gas_limit as i64,
                value,
                code_path,
                arguments,
                result_address_handle,
            );

            // dest
        }
    }

    // Set storage of any account
    fn set_storage(
        &self,
        address: Self::ManagedBufferHandle,
        key: Self::ManagedBufferHandle,
        value: Self::ManagedBufferHandle,
    ) {
        unsafe {
            setStorage(address, key, value);
        }
    }

    // Get storage of any account
    fn get_storage(
        &self,
        address: Self::ManagedBufferHandle,
        key: Self::ManagedBufferHandle,
        result_value: Self::ManagedBufferHandle,
    ) {
        unsafe {
            // let mut dest = ManagedBuffer::new();

            getStorage(address, key, result_value);

            // dest
        }
    }

    // Start a prank: set the caller address for contract calls until stop_prank
    fn start_prank(&self, address: Self::ManagedBufferHandle) {
        unsafe {
            startPrank(address);
        }
    }

    // Stop a prank: reset the caller address
    fn stop_prank(&self) {
        unsafe {
            stopPrank();
        }
    }

    fn assume(&self, p: bool) {
        unsafe {
            assumeBool(p);
        }
    }

    fn assert(&self, p: bool) {
        unsafe {
            assertBool(p);
        }
    }

    fn set_block_timestamp(&self, timestamp: u64) {
        unsafe {
            setBlockTimestamp(timestamp as i64);
        }
    }

    fn set_balance(&self, address: Self::ManagedBufferHandle, value: Self::BigIntHandle) {
        unsafe {
            setExternalBalance(address, value);
        }
    }

    fn set_esdt_balance(
        &self,
        address: Self::ManagedBufferHandle,
        token_id: Self::ManagedBufferHandle,
        value: Self::BigIntHandle,
    ) {
        unsafe {
            setESDTExternalBalance(address, token_id, value);
        }
    }
}
