use super::{BigIntUncallable, BigUintUncallable, EllipticCurveUncallable};
use crate::api::SendApi;
use crate::types::{
    Address, ArgBuffer, BoxedBytes, CodeMetadata, EsdtTokenPayment, TokenIdentifier,
};
use alloc::vec::Vec;

impl SendApi for super::UncallableApi {
    type AmountType = BigUintUncallable;
    type ProxyBigInt = BigIntUncallable;
    type ProxyEllipticCurve = EllipticCurveUncallable;
    type ProxyStorage = Self;

    fn get_sc_address(&self) -> Address {
        unreachable!()
    }

    fn get_gas_left(&self) -> u64 {
        unreachable!()
    }

    fn get_esdt_token_data(
        &self,
        _address: &Address,
        _token: &TokenIdentifier,
        _nonce: u64,
    ) -> crate::types::EsdtTokenData<Self::AmountType> {
        unreachable!()
    }

    fn direct_egld(&self, _to: &Address, _amount: &BigUintUncallable, _data: &[u8]) {
        unreachable!()
    }

    fn direct_egld_execute(
        &self,
        _to: &Address,
        _amount: &BigUintUncallable,
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_execute(
        &self,
        _to: &Address,
        _token: &TokenIdentifier,
        _amount: &BigUintUncallable,
        _gas: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_esdt_nft_execute(
        &self,
        _to: &Address,
        _token: &TokenIdentifier,
        _nonce: u64,
        _amount: &BigUintUncallable,
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn direct_multi_esdt_transfer_execute(
        &self,
        _to: &Address,
        _tokens: &[EsdtTokenPayment<BigUintUncallable>],
        _gas_limit: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Result<(), &'static [u8]> {
        unreachable!()
    }

    fn async_call_raw(&self, _to: &Address, _amount: &BigUintUncallable, _data: &[u8]) -> ! {
        unreachable!()
    }

    fn deploy_contract(
        &self,
        _gas: u64,
        _amount: &BigUintUncallable,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        unreachable!()
    }

    fn deploy_from_source_contract(
        &self,
        _gas: u64,
        _amount: &BigUintUncallable,
        _source_contract_address: &Address,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) -> Option<Address> {
        unreachable!()
    }

    fn upgrade_contract(
        &self,
        _sc_address: &Address,
        _gas: u64,
        _amount: &BigUintUncallable,
        _code: &BoxedBytes,
        _code_metadata: CodeMetadata,
        _arg_buffer: &ArgBuffer,
    ) {
        unreachable!()
    }

    fn execute_on_dest_context_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUintUncallable,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        unreachable!()
    }

    fn execute_on_dest_context_raw_custom_result_range<F>(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUintUncallable,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
        _range_closure: F,
    ) -> Vec<BoxedBytes>
    where
        F: FnOnce(usize, usize) -> (usize, usize),
    {
        unreachable!()
    }

    fn execute_on_dest_context_by_caller_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUintUncallable,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        unreachable!()
    }

    fn execute_on_same_context_raw(
        &self,
        _gas: u64,
        _address: &Address,
        _value: &BigUintUncallable,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) {
        unreachable!()
    }

    fn storage_store_tx_hash_key(&self, _data: &[u8]) {
        unreachable!()
    }

    fn storage_load_tx_hash_key(&self) -> BoxedBytes {
        unreachable!()
    }

    fn call_local_esdt_built_in_function(
        &self,
        _gas: u64,
        _function: &[u8],
        _arg_buffer: &ArgBuffer,
    ) -> Vec<BoxedBytes> {
        unreachable!()
    }

    fn sell_nft(
        &self,
        _nft_id: &TokenIdentifier,
        _nft_nonce: u64,
        _nft_amount: &Self::AmountType,
        _buyer: &Address,
        _payment_token: &TokenIdentifier,
        _payment_nonce: u64,
        _payment_amount: &Self::AmountType,
    ) -> Self::AmountType {
        unreachable!()
    }
}
