use crate::{
    api::{ManagedTypeApi, SendApi, SendApiImpl},
    types::{
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
    fn direct_egld<M, D>(&self, _to: &ManagedAddress<M>, _amount: &BigUint<M>, _data: D)
    where
        M: ManagedTypeApi,
        D: Into<ManagedBuffer<M>>,
    {
        unreachable!()
    }

    fn direct_egld_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _amount: &BigUint<M>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_execute<M: ManagedTypeApi>(
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

    fn direct_esdt_nft_execute<M: ManagedTypeApi>(
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

    fn direct_multi_esdt_transfer_execute<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<M>,
        _payments: &ManagedVec<M, EsdtTokenPayment<M>>,
        _gas_limit: u64,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
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

    fn create_async_call_raw<M: ManagedTypeApi>(
        &self,
        _to: &ManagedAddress<Self>,
        _amount: &BigUint<Self>,
        _endpoint_name: &ManagedBuffer<Self>,
        _success: &'static [u8],
        _error: &'static [u8],
        _gas: u64,
        _extra_gas_for_callback: u64,
        _arg_buffer: &ManagedArgBuffer<Self>,
    ) -> Result<(), &'static [u8]> {
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

    fn execute_on_dest_context_raw_custom_result_range<M, F>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _value: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
        _range_closure: F,
    ) -> ManagedVec<M, ManagedBuffer<M>>
    where
        M: ManagedTypeApi,
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        unreachable!()
    }

    fn execute_on_dest_context_by_caller_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _to: &ManagedAddress<M>,
        _value: &BigUint<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
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

    fn execute_on_dest_context_readonly_raw<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _address: &ManagedAddress<M>,
        _endpoint_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }

    fn storage_store_tx_hash_key<M: ManagedTypeApi>(&self, _data: &ManagedBuffer<M>) {
        unreachable!()
    }

    fn storage_load_tx_hash_key<M: ManagedTypeApi>(&self) -> ManagedBuffer<M> {
        unreachable!()
    }

    fn call_local_esdt_built_in_function<M: ManagedTypeApi>(
        &self,
        _gas: u64,
        _function_name: &ManagedBuffer<M>,
        _arg_buffer: &ManagedArgBuffer<M>,
    ) -> ManagedVec<M, ManagedBuffer<M>> {
        unreachable!()
    }
}
