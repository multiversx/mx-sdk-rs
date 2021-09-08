use crate::api::SendApi;
use crate::types::{
    ArgBuffer, BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer,
    ManagedBuffer, ManagedInto, ManagedVec, TokenIdentifier,
};

impl SendApi for super::UncallableApi {
    type ProxyTypeManager = Self;
    type ProxyStorage = Self;
    type ErrorApi = Self;
    type BlockchainApi = Self;

    fn type_manager(&self) -> Self::ProxyTypeManager {
        unreachable!()
    }

    fn error_api(&self) -> Self::ErrorApi {
        unreachable!()
    }

    fn blockchain(&self) -> Self::BlockchainApi {
        unreachable!()
    }

    fn direct_egld<D>(
        &self,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _data: D,
    ) where
        D: ManagedInto<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    {
        unreachable!()
    }

    fn direct_egld_execute(
        &self,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_execute(
        &self,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _token: &TokenIdentifier<Self::ProxyTypeManager>,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _gas: u64,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_nft_execute(
        &self,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _token: &TokenIdentifier<Self::ProxyTypeManager>,
        _nonce: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _payments: &ManagedVec<Self::ProxyTypeManager, EsdtTokenPayment<Self::ProxyTypeManager>>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn async_call_raw(
        &self,
        to: &ManagedAddress<Self::ProxyTypeManager>,
        amount: &BigUint<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ! {
        unreachable!()
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _code: &ManagedBuffer<Self::ProxyTypeManager>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> (
        ManagedAddress<Self::ProxyTypeManager>,
        ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    ) {
        unreachable!()
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _source_contract_address: &ManagedAddress<Self::ProxyTypeManager>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> (
        ManagedAddress<Self::ProxyTypeManager>,
        ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>,
    ) {
        unreachable!()
    }

    fn upgrade_contract(
        &self,
        _sc_address: &ManagedAddress<Self::ProxyTypeManager>,
        _gas: u64,
        _amount: &BigUint<Self::ProxyTypeManager>,
        _code: &ManagedBuffer<Self::ProxyTypeManager>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _value: &BigUint<Self::ProxyTypeManager>,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _value: &BigUint<Self::ProxyTypeManager>,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
        _range_closure: F,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        unreachable!()
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _value: &BigUint<Self::ProxyTypeManager>,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self::ProxyTypeManager>,
        _value: &BigUint<Self::ProxyTypeManager>,
        _endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<Self::ProxyTypeManager>,
        endpoint_name: &ManagedBuffer<Self::ProxyTypeManager>,
        arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn storage_store_tx_hash_key(&self, _data: &ManagedBuffer<Self::ProxyTypeManager>) {
        unreachable!()
    }

    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self::ProxyTypeManager> {
        unreachable!()
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        _function_name: &ManagedBuffer<Self::ProxyTypeManager>,
        _arg_buffer: &ManagedArgBuffer<Self::ProxyTypeManager>,
    ) -> ManagedVec<Self::ProxyTypeManager, ManagedBuffer<Self::ProxyTypeManager>> {
        unreachable!()
    }

    fn sell_nft(
        &self,
        _nft_id: &TokenIdentifier<Self::ProxyTypeManager>,
        _nft_nonce: u64,
        _nft_amount: &BigUint<Self::ProxyTypeManager>,
        _buyer: &ManagedAddress<Self::ProxyTypeManager>,
        _payment_token: &TokenIdentifier<Self::ProxyTypeManager>,
        _payment_nonce: u64,
        _payment_amount: &BigUint<Self::ProxyTypeManager>,
    ) -> BigUint<Self::ProxyTypeManager> {
        unreachable!()
    }
}
