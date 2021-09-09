use elrond_codec::{TopDecode, TopEncode};

use super::{BlockchainApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi};
use crate::{
    types::{
        managed_vec_from_slice_of_boxed_bytes, ArgBuffer, BigUint, CodeMetadata, EsdtTokenPayment,
        ManagedAddress, ManagedArgBuffer, ManagedBuffer, ManagedFrom, ManagedInto, ManagedVec,
        TokenIdentifier, Vec,
    },
    HexCallDataSerializer,
};

pub const ESDT_TRANSFER_STRING: &[u8] = b"ESDTTransfer";
pub const ESDT_NFT_TRANSFER_STRING: &[u8] = b"ESDTNFTTransfer";
pub const ESDT_MULTI_TRANSFER_STRING: &[u8] = b"MultiESDTNFTTransfer";

const PERCENTAGE_TOTAL: u64 = 10_000;

/// API that groups methods that either send EGLD or ESDT, or that call other contracts.
pub trait SendApi: ManagedTypeApi + BlockchainApi + Clone + Sized {
    /// Sends EGLD to a given address, directly.
    /// Used especially for sending EGLD to regular accounts.
    fn direct_egld<D>(&self, to: &ManagedAddress<Self>, amount: &BigUint<Self>, data: D)
    where
        D: ManagedInto<Self, ManagedBuffer<Self>>;

    /// Sends EGLD to an address (optionally) and executes like an async call, but without callback.
    fn direct_egld_execute(
        &self,
        to: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT to an address and executes like an async call, but without callback.
    fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        amount: &BigUint<Self>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]>;

    /// Sends ESDT NFT to an address and executes like an async call, but without callback.
    #[allow(clippy::too_many_arguments)]
    fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
        amount: &BigUint<Self>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]>;

    fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<Self>,
        payments: &ManagedVec<Self, EsdtTokenPayment<Self>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]>;

    /// Sends an asynchronous call to another contract.
    /// Calling this method immediately terminates tx execution.
    /// Using it directly is generally discouraged.
    ///
    /// The data is expected to be of the form `functionName@<arg1-hex>@<arg2-hex>@...`.
    /// Use a `HexCallDataSerializer` to prepare this field.
    fn async_call_raw(
        &self,
        to: &ManagedAddress<Self>,
        amount: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> !;

    /// Performs a simple ESDT/NFT transfer, but via async call.  
    /// As with any async call, this immediately terminates the execution of the current call.  
    /// So only use as the last call in your endpoint.  
    /// If you want to perform multiple transfers, use `self.send().transfer_multiple_esdt_via_async_call()` instead.  
    /// Note that EGLD can NOT be transfered with this function.  
    fn transfer_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        nonce: u64,
        amount: &BigUint<Self>,
        data: D,
    ) -> !
    where
        D: ManagedInto<Self, ManagedBuffer<Self>>,
    {
        let data_buf: ManagedBuffer<Self> = data.managed_into(self.clone());
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.clone());
        arg_buffer.push_arg(token);
        if nonce == 0 {
            arg_buffer.push_arg(amount);
            if !data_buf.is_empty() {
                arg_buffer.push_arg_raw(data_buf);
            }

            self.async_call_raw(
                to,
                &BigUint::zero(self.clone()),
                &ManagedBuffer::new_from_bytes(self.clone(), ESDT_TRANSFER_STRING),
                &arg_buffer,
            )
        } else {
            arg_buffer.push_arg(nonce);
            arg_buffer.push_arg(amount);
            arg_buffer.push_arg(to);
            if !data_buf.is_empty() {
                arg_buffer.push_arg_raw(data_buf);
            }

            self.async_call_raw(
                &self.get_sc_address(),
                &BigUint::zero(self.clone()),
                &ManagedBuffer::new_from_bytes(self.clone(), ESDT_NFT_TRANSFER_STRING),
                &arg_buffer,
            )
        }
    }

    fn transfer_multiple_esdt_via_async_call<D>(
        &self,
        to: &ManagedAddress<Self>,
        payments: &ManagedVec<Self, EsdtTokenPayment<Self>>,
        data: D,
    ) -> !
    where
        D: ManagedInto<Self, ManagedBuffer<Self>>,
    {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.clone());
        arg_buffer.push_arg(to);
        arg_buffer.push_arg(payments.len());

        for payment in payments.into_iter() {
            // TODO: check that `!token_name.is_egld()` or let Arwen throw the error?
            arg_buffer.push_arg(payment.token_name);
            arg_buffer.push_arg(payment.token_nonce);
            arg_buffer.push_arg(payment.amount);
        }
        let data_buf: ManagedBuffer<Self> = data.managed_into(self.clone());
        if !data_buf.is_empty() {
            arg_buffer.push_arg_raw(data_buf);
        }

        self.async_call_raw(
            &self.get_sc_address(),
            &BigUint::zero(self.clone()),
            &ManagedBuffer::new_from_bytes(self.clone(), ESDT_MULTI_TRANSFER_STRING),
            &arg_buffer,
        );
    }

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self>,
        code: &ManagedBuffer<Self>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Option<ManagedAddress<Self>>;

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<Self>,
        source_contract_address: &ManagedAddress<Self>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Option<ManagedAddress<Self>>;

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<Self>,
        gas: u64,
        amount: &BigUint<Self>,
        code: &ManagedBuffer<Self>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<Self>,
    );

    /// Same shard, in-line execution of another contract.
    fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>;

    /// Same shard, in-line execution of another contract.
    /// Allows the contract to specify which result range to extract as sync call result.
    /// This is a workaround to handle nested sync calls.
    /// Please do not use this method unless there is absolutely no other option.
    /// Will be eliminated after some future Arwen hook redesign.
    /// `range_closure` takes the number of results before, the number of results after,
    /// and is expected to return the start index (inclusive) and end index (exclusive).
    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        gas: u64,
        address: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
        range_closure: F,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>
    where
        F: FnOnce(usize, usize) -> (usize, usize);

    fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>;

    fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self>,
        value: &BigUint<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    );

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self>,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>;

    /// Used to store data between async call and callback.
    fn storage_store_tx_hash_key(&self, data: &ManagedBuffer<Self>);

    /// Used to store data between async call and callback.
    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self>;

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    /// You should never have to call this function directly.
    /// Use the other specific methods instead.
    fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<Self>,
        arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>;
}
