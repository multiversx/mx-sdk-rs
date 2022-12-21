use core::marker::PhantomData;

use crate::{
    api::{const_handles, use_raw_handle, BlockchainApiImpl, CallTypeApi, SendApiImpl},
    types::{
        BigUint, CodeMetadata, EsdtTokenPayment, ManagedAddress, ManagedArgBuffer, ManagedBuffer,
        ManagedType, ManagedVec, TokenIdentifier,
    },
};

#[derive(Default)]
pub struct SendRawWrapper<A>
where
    A: CallTypeApi,
{
    _phantom: PhantomData<A>,
}

impl<A> SendRawWrapper<A>
where
    A: CallTypeApi,
{
    pub(crate) fn new() -> Self {
        SendRawWrapper {
            _phantom: PhantomData,
        }
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        A::send_api_impl().transfer_value_legacy(
            &to.to_address(),
            amount,
            &data.into().to_boxed_bytes(),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, amount: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        let _ = A::send_api_impl().transfer_value_execute(
            to,
            amount,
            0,
            &data.into(),
            &ManagedArgBuffer::new(),
        );
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_value_execute_legacy(
            &to.to_address(),
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_value_execute(to, amount, gas_limit, endpoint_name, arg_buffer)
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn transfer_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_esdt_execute(
            to,
            token,
            amount,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn transfer_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_esdt_execute_legacy(
            &to.to_address(),
            token,
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_esdt_nft_execute(
            to,
            token,
            nonce,
            amount,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        amount: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().transfer_esdt_nft_execute_legacy(
            &to.to_address(),
            token,
            nonce,
            amount,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        let payments_vec = payments.clone().into_vec();
        A::send_api_impl().multi_transfer_esdt_nft_execute_legacy(
            &to.to_address(),
            &payments_vec,
            gas_limit,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), &'static [u8]> {
        A::send_api_impl().multi_transfer_esdt_nft_execute(
            to,
            payments,
            gas_limit,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
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
            &arg_buffer.into(),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ! {
        A::send_api_impl().async_call_raw(to, amount, endpoint_name, arg_buffer)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        amount: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        serialized_callback_closure_args: &ManagedBuffer<A>,
    ) {
        A::send_api_impl().create_async_call_raw(
            to.get_handle(),
            amount.get_handle(),
            endpoint_name.get_handle(),
            arg_buffer.get_handle(),
            success_callback,
            error_callback,
            gas,
            extra_gas_for_callback,
            serialized_callback_closure_args.get_handle(),
        )
    }

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        A::send_api_impl().deploy_contract(gas, amount, code, code_metadata, arg_buffer)
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn deploy_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        A::send_api_impl().deploy_contract_legacy(
            gas,
            amount,
            &code.to_boxed_bytes(),
            code_metadata,
            &arg_buffer.into(),
        )
    }

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        A::send_api_impl().deploy_from_source_contract(
            gas,
            amount,
            source_contract_address,
            code_metadata,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn deploy_from_source_contract(
        &self,
        gas: u64,
        amount: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        A::send_api_impl().deploy_from_source_contract_legacy(
            gas,
            amount,
            &source_contract_address.to_address(),
            code_metadata,
            &arg_buffer.into(),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn upgrade_from_source_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        amount: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().upgrade_from_source_contract(
            sc_address,
            gas,
            amount,
            source_contract_address,
            code_metadata,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn upgrade_from_source_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        amount: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().upgrade_from_source_contract_legacy(
            &sc_address.to_address(),
            gas,
            amount,
            &source_contract_address.to_address(),
            code_metadata,
            &arg_buffer.into(),
        )
    }

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().upgrade_contract(
            sc_address,
            gas,
            amount,
            code,
            code_metadata,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        amount: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().upgrade_contract_legacy(
            &sc_address.to_address(),
            gas,
            amount,
            &code.to_boxed_bytes(),
            code_metadata,
            &arg_buffer.into(),
        )
    }

    /// Same shard, in-line execution of another contract.
    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_raw(
            gas,
            address,
            value,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_raw_legacy(
            gas,
            &address.to_address(),
            value,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_same_context_raw(
            gas,
            address,
            value,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_same_context_raw_legacy(
            gas,
            &address.to_address(),
            value,
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    /// Same shard, in-line execution of another contract.
    #[cfg(not(feature = "ei-unmanaged"))]
    pub fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_readonly_raw(
            gas,
            address,
            endpoint_name,
            arg_buffer,
        )
    }

    #[cfg(feature = "ei-unmanaged")]
    pub fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        A::send_api_impl().execute_on_dest_context_readonly_raw_legacy(
            gas,
            &address.to_address(),
            &endpoint_name.to_boxed_bytes(),
            &crate::types::heap::ArgBuffer::from(arg_buffer),
        )
    }

    /// Allows synchronously calling a local function by name. Execution is resumed afterwards.
    pub fn call_local_esdt_built_in_function(
        &self,
        gas: u64,
        function_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        // account-level built-in function, so the destination address is the contract itself
        let own_address_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::blockchain_api_impl().load_sc_address_managed(own_address_handle.clone());

        let results = A::send_api_impl().execute_on_dest_context_raw(
            gas,
            &ManagedAddress::from_handle(own_address_handle),
            &BigUint::zero(),
            function_name,
            arg_buffer,
        );

        self.clean_return_data();

        results
    }

    pub fn clean_return_data(&self) {
        A::send_api_impl().clean_return_data()
    }
}
