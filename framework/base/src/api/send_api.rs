use super::{BlockchainApi, HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl};
use crate::types::{
    heap::{Address, ArgBuffer, BoxedBytes},
    BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
    ManagedVec, TokenIdentifier,
};

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
    fn transfer_value_legacy<M>(&self, to: &Address, amount: &BigUint<M>, data: &BoxedBytes)
    where
        M: ManagedTypeApi;

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn transfer_value_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    fn transfer_value_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT to an address and executes like an async call, but without callback.
    fn transfer_esdt_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    fn transfer_esdt_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    fn transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    #[allow(clippy::too_many_arguments)]
    fn transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    fn multi_transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    fn multi_transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        payments: &[EsdtTokenPayment<M>],
        gas_limit: u64,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]>;

    /// Sends an asynchronous call to another contract.
    /// Calling this method immediately terminates tx execution.
    /// Using it directly is generally discouraged.
    ///
    /// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
    /// Use a `HexCallDataSerializer` to prepare this field.
    fn async_call_raw<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> !;

    fn async_call_raw_legacy<M: ManagedTypeApi>(
        &self,
        to: &Address,
        amount: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> !;

    #[allow(clippy::too_many_arguments)]
    fn create_async_call_raw(
        &self,
        to: Self::ManagedBufferHandle,
        amount: Self::BigIntHandle,
        endpoint_name: Self::ManagedBufferHandle,
        arg_buffer: Self::ManagedBufferHandle,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        callback_closure: Self::ManagedBufferHandle,
    );

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    fn deploy_contract<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>);

    fn deploy_contract_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>);

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    fn deploy_from_source_contract<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>);

    fn deploy_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>);

    fn upgrade_from_source_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    );

    fn upgrade_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &Address,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    );

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        gas: u64,
        amount: &BigUint<M>,
        code: &ManagedBuffer<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
    );

    fn upgrade_contract_legacy<M: ManagedTypeApi>(
        &self,
        sc_address: &Address,
        gas: u64,
        amount: &BigUint<M>,
        code: &BoxedBytes,
        code_metadata: CodeMetadata,
        arg_buffer: &ArgBuffer,
    );

    /// Same shard, in-line execution of another contract.
    fn execute_on_dest_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_dest_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &Address,
        value: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_same_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_same_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &Address,
        value: &BigUint<M>,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_dest_context_readonly_raw_legacy<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &Address,
        endpoint_name: &BoxedBytes,
        arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn clean_return_data(&self);

    fn delete_from_return_data(&self, index: usize);
}
