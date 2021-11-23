use crate::{
    api::SendApi,
    types::{
        BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedVec, TokenIdentifier,
    },
};

impl SendApi for super::UncallableApi {
    fn direct_egld<D>(&self, _to: &ManagedAddress<Self>, _amount: &BigUint<Self>, _data: D)
    where
        D: Into<ManagedBuffer<Self>>,
    {
        unreachable!()
    }

    fn direct_egld_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _amount: &BigUint<Self>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _amount: &BigUint<Self>,
        _gas: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_nft_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
        _amount: &BigUint<Self>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        _to: &ManagedAddress<Self>,
        _payments: &ManagedVec<Self, EsdtTokenPayment<Self>>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn async_call_raw(
        &self,
        _to: &ManagedAddress<Self>,
        _amount: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ! {
        unreachable!()
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self>,
        _code: &ManagedBuffer<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        unreachable!()
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount: &BigUint<Self>,
        _source_contract_address: &ManagedAddress<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> (ManagedAddress<Self>, ManagedVec<Self, ManagedBuffer<Self>>) {
        unreachable!()
    }

    fn upgrade_from_source_contract(
        &self,
        _sc_address: &ManagedAddress<Self>,
        _gas: u64,
        _amount: &BigUint<Self>,
        _source_contract_address: &ManagedAddress<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) {
        unreachable!()
    }

    fn upgrade_contract(
        &self,
        _sc_address: &ManagedAddress<Self>,
        _gas: u64,
        _amount: &BigUint<Self>,
        _code: &ManagedBuffer<Self>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) {
        unreachable!()
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        unreachable!()
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
        _range_closure: F,
    ) -> ManagedVec<Self, ManagedBuffer<Self>>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        unreachable!()
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        unreachable!()
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _to: &ManagedAddress<Self>,
        _value: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        unreachable!()
    }

    fn execute_on_dest_context_readonly_raw(
        &self,
        _gas: u64,
        _address: &ManagedAddress<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        unreachable!()
    }

    fn storage_store_tx_hash_key(&self, _data: &ManagedBuffer<Self>) {
        unreachable!()
    }

    fn storage_load_tx_hash_key(&self) -> ManagedBuffer<Self> {
        unreachable!()
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        _function_name: &ManagedBuffer<Self>,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> ManagedVec<Self, ManagedBuffer<Self>> {
        unreachable!()
    }
}
