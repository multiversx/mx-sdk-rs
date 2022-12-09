use crate::{
    api::{ManagedTypeApi, SendApi, SendApiImpl},
    types::{
        heap::{Address, ArgBuffer, BoxedBytes},
        BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedVec, TokenIdentifier,
    },
};

use super::UncallableApi;

impl SendApi for UncallableApi {
    type SendApiImpl = UncallableApi;

    fn send_api_impl() -> Self::SendApiImpl {
        unreachable!()
    }
}

impl SendApiImpl for UncallableApi {
    fn transfer_value_legacy<M>(&self, _to: &Address, _amount: &BigUint<M>, _data: &BoxedBytes)
    where
        M: ManagedTypeApi,
    {
        unreachable!()
    }

    fn transfer_value_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn transfer_value_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn transfer_esdt_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
        _amount: &BigUint<M>,
        _gas: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn transfer_esdt_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _token: &TokenIdentifier<M>,
        _amount: &BigUint<M>,
        _gas: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn multi_transfer_esdt_nft_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn multi_transfer_esdt_nft_execute_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _payments: &[EsdtTokenPayment<M>],
        _gas_limit: u64,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn async_call_raw<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _amount: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ! {
        unreachable!()
    }

    fn async_call_raw_legacy<M: ManagedTypeApi>(
        &self,
        _to: &Address,
        _amount: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ! {
        unreachable!()
    }

    fn create_async_call_raw(
        &self,
        _to: Self::ManagedBufferHandle,
        _amount: Self::BigIntHandle,
        _endpoint_name: Self::ManagedBufferHandle,
        _arg_buffer: Self::ManagedBufferHandle,
        _success_callback: &'static str,
        _error_callback: &'static str,
        _gas: u64,
        _extra_gas_for_callback: u64,
        _callback_closure: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }

    fn deploy_contract<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &ManagedBuffer<M>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unreachable!()
    }

    fn deploy_contract_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unreachable!()
    }

    fn deploy_from_source_contract<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &ManagedAddress<M>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unreachable!()
    }

    fn deploy_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> (ManagedAddress<M>, ManagedVec<M, ManagedBuffer<M>>) {
        unreachable!()
    }

    fn upgrade_from_source_contract<M: ManagedTypeApi>(
        &self,
        _sc_address: &ManagedAddress<M>,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &ManagedAddress<M>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) {
        unreachable!()
    }

    fn upgrade_from_source_contract_legacy<M: ManagedTypeApi>(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUint<M>,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        unreachable!()
    }

    fn upgrade_contract<M: ManagedTypeApi>(
        &self,
        _sc_address: &ManagedAddress<M>,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &ManagedBuffer<M>,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) {
        unreachable!()
    }

    fn upgrade_contract_legacy<M: ManagedTypeApi>(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUint<M>,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        unreachable!()
    }

    fn execute_on_dest_context_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _value: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn execute_on_dest_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &Address,
        _value: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn execute_on_same_context_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _value: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn execute_on_same_context_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &Address,
        _value: &BigUint<M>,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _address: &ManagedAddress<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn execute_on_dest_context_readonly_raw_legacy<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _address: &Address,
        _endpoint_name: &BoxedBytes,
        _arg_buffer: &ArgBuffer,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn clean_return_data(&self) {
        unreachable!();
    }

    fn delete_from_return_data(&self, _index: usize) {
        unreachable!();
    }
}
