use super::{ErrorApi, ErrorApiImpl, HandleTypeInfo, ManagedTypeApi};

const TEST_API_ERROR_MSG: &str = "cannot call the test API in this context";

pub trait TestApi: ManagedTypeApi + ErrorApi {
    type TestApiImpl: TestApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn test_api_impl() -> Self::TestApiImpl;
}

#[allow(unused_variables)]
pub trait TestApiImpl: HandleTypeInfo + ErrorApi {
    fn create_account(
        &self,
        address: Self::ManagedBufferHandle,
        nonce: u64,
        balance: Self::BigIntHandle,
    ) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn register_new_address(
        &self,
        owner: Self::ManagedBufferHandle,
        nonce: u64,
        new_address: Self::ManagedBufferHandle,
    ) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
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
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    // Set storage of any account
    fn set_storage(
        &self,
        address: Self::ManagedBufferHandle,
        key: Self::ManagedBufferHandle,
        value: Self::ManagedBufferHandle,
    ) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    // Get storage of any account
    fn get_storage(
        &self,
        address: Self::ManagedBufferHandle,
        key: Self::ManagedBufferHandle,
        result_value: Self::ManagedBufferHandle,
    ) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    // Start a prank: set the caller address for contract calls until stop_prank
    fn start_prank(&self, address: Self::ManagedBufferHandle) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    // Stop a prank: reset the caller address
    fn stop_prank(&self) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn assume(&self, p: bool) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn assert(&self, p: bool) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn set_block_timestamp(&self, timestamp: u64) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn set_balance(&self, address: Self::ManagedBufferHandle, value: Self::BigIntHandle) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }

    fn set_esdt_balance(
        &self,
        address: Self::ManagedBufferHandle,
        token_id: Self::ManagedBufferHandle,
        value: Self::BigIntHandle,
    ) {
        Self::error_api_impl().signal_error(TEST_API_ERROR_MSG.as_bytes());
    }
}
