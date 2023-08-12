use super::{BlockchainApi, HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl, RawHandle};

pub trait SendApi: ManagedTypeApi + BlockchainApi {
    type SendApiImpl: SendApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn send_api_impl() -> Self::SendApiImpl;
}

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApiImpl: ManagedTypeApiImpl {
    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn transfer_value_execute(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]>;

    fn multi_transfer_esdt_nft_execute(
        &self,
        to_handle: RawHandle,
        payments_handle: RawHandle,
        gas_limit: u64,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> Result<(), &'static [u8]>;

    /// Sends an asynchronous call to another contract.
    /// Calling this method immediately terminates tx execution.
    /// Using it directly is generally discouraged.
    fn async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    ) -> !;

    #[allow(clippy::too_many_arguments)]
    fn create_async_call_raw(
        &self,
        to_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        callback_closure: RawHandle,
    );

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    #[allow(clippy::too_many_arguments)]
    fn deploy_contract(
        &self,
        gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    );

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    #[allow(clippy::too_many_arguments)]
    fn deploy_from_source_contract(
        &self,
        gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        new_address_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn upgrade_from_source_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        egld_value_handle: RawHandle,
        source_contract_address_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    );

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract(
        &self,
        sc_address_handle: RawHandle,
        gas: u64,
        egld_value_handle: RawHandle,
        code_handle: RawHandle,
        code_metadata_handle: RawHandle,
        arg_buffer_handle: RawHandle,
    );

    /// Same shard, in-line execution of another contract.
    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        egld_value_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address_handle: RawHandle,
        endpoint_name_handle: RawHandle,
        arg_buffer_handle: RawHandle,
        result_handle: RawHandle,
    );

    fn clean_return_data(&self);

    fn delete_from_return_data(&self, index: usize);
}
