use super::{BlockchainApi, ManagedTypeApi};
use crate::types::{
    BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
    ManagedVec, TokenIdentifier,
};

pub trait SendApi: ManagedTypeApi + BlockchainApi {
    type SendApiImpl: SendApiImpl;

    fn send_api_impl() -> Self::SendApiImpl;
}

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApiImpl {
    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    fn direct_egld<M, D>(&self, to: &ManagedAddress<M>, amount: &BigUint<M>, data: D)
    where
        M: ManagedTypeApi,
        D: Into<ManagedBuffer<M>>;

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn direct_egld_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT to an address and executes like an async call, but without callback.
    fn direct_esdt_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    fn direct_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
        amount: &BigUint<M>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]>;

    fn direct_multi_esdt_transfer_execute<M: ManagedTypeApi>(
        &self,
        to: &ManagedAddress<M>,
        payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
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

    fn upgrade_from_source_contract<M: ManagedTypeApi>(
        &self,
        sc_address: &ManagedAddress<M>,
        gas: u64,
        amount: &BigUint<M>,
        source_contract_address: &ManagedAddress<M>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<M>,
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

    /// Same shard, in-line execution of another contract.
    fn execute_on_dest_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    /// Same shard, in-line execution of another contract.
    /// Allows the contract to specify which result range to extract as sync call result.
    /// This is a workaround to handle nested sync calls.
    /// Please do not use this method unless there is absolutely no other option.
    /// Will be eliminated after some future Arwen hook redesign.
    /// `range_closure` takes the number of results before, the number of results after,
    /// and is expected to return the start index (inclusive) and end index (exclusive).
    fn execute_on_dest_context_raw_custom_result_range<M, F>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
        range_closure: F,
    ) -> ManagedVec<M, ManagedBuffer<M>>
    where
        M: ManagedTypeApi,
        F: FnOnce(usize, usize) -> (usize, usize);

    fn execute_on_dest_context_by_caller_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_same_context_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        value: &BigUint<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        gas: u64,
        address: &ManagedAddress<M>,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;

    /// Used to store data between async call and callback.
    fn storage_store_tx_hash_key<M: ManagedTypeApi>(&self, data: &ManagedBuffer<M>);

    /// Used to store data between async call and callback.
    fn storage_load_tx_hash_key<M: ManagedTypeApi>(&self) -> ManagedBuffer<M>;

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    fn call_local_esdt_built_in_function<M: ManagedTypeApi>(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<M>,
        arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>>;
}
