use core::marker::PhantomData;

use crate::{
    api::{BlockchainApi, CallTypeApi, SendApiImpl, StorageReadApi},
    types::{
        heap::ArgBuffer, BigUint, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer,
        ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct SendRawWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    _phantom: PhantomData<A>,
}

impl<A> SendRawWrapper<A>
where
    A: CallTypeApi + StorageReadApi + BlockchainApi,
{
    pub(crate) fn new() -> Self {
        SendRawWrapper {
            _phantom: PhantomData,
        }
    }
    #[cfg(feature = "alloc")]
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        A::send_api_impl().direct_egld_legacy(
            &to.to_address(),
            amount,
            &data.into().to_boxed_bytes(),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        A::send_api_impl().direct_egld(to, amount, data)
    }

    #[cfg(feature = "alloc")]
    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_egld_execute_legacy(
            &to.to_address(),
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_egld_execute(to, amount, gas_limit, endpoint_name, arg_buffer)
    }

    #[cfg(feature = "alloc")]
    pub fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_esdt_execute_legacy(
            &to.to_address(),
            token,
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn direct_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_esdt_execute(
            to,
            token,
            amount,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "alloc")]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_esdt_nft_execute_legacy(
            &to.to_address(),
            token,
            nonce,
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    #[allow(clippy::too_many_arguments)]
    pub fn direct_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_esdt_nft_execute(
            to,
            token,
            nonce,
            amount,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "alloc")]
    pub fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        let payments_vec = payments.clone().into_vec();
        A::send_api_impl().direct_multi_esdt_transfer_execute_legacy(
            &to.to_address(),
            &payments_vec,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn direct_multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().direct_multi_esdt_transfer_execute(
            to,
            payments,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "alloc")]
    pub fn async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ! {
        A::send_api_impl().async_call_raw_legacy(
            &to.to_address(),
            amount,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ! {
        A::send_api_impl().async_call_raw(to, amount, endpoint_name, arg_buffer)
    }

    #[cfg(feature = "alloc")]
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().call_local_esdt_built_in_function_legacy(
            gas,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().call_local_esdt_built_in_function(gas, endpoint_name, arg_buffer)
    }

    pub fn clean_return_data(&self) {
        A::send_api_impl().clean_return_data()
    }

    #[cfg(feature = "alloc")]
    pub fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_by_caller_raw_legacy(
            gas,
            &address.to_address(),
            value,
            &endpoint_name.to_boxed_bytes(),
            &ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "alloc"))]
    pub fn execute_on_dest_context_by_caller_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_by_caller_raw(
            gas,
            address,
            value,
            endpoint_name,
            arg_buffer,
        )
    }
}
