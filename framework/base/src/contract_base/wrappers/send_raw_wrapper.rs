use core::marker::PhantomData;

use crate::{
    api::{
        const_handles, use_raw_handle, BigIntApiImpl, BlockchainApiImpl, CallTypeApi,
        HandleConstraints, ManagedBufferApiImpl, RawHandle, SendApiImpl, StaticVarApiImpl,
    },
    types::{
        BigUint, CodeMetadata, EgldOrEsdtTokenPayment, EsdtTokenPayment, ManagedAddress,
        ManagedArgBuffer, ManagedBuffer, ManagedType, ManagedVec, TokenIdentifier,
    },
};

/// Wraps the result of a sync call.
///
/// Is used primarily in result handlers.
pub struct SyncCallRawResult<Api>(pub ManagedVec<Api, ManagedBuffer<Api>>)
where
    Api: CallTypeApi;

/// Wraps the result of a fallible sync call (one that returns the error instead of stopping execution).
///
/// Is used primarily in result handlers.
pub enum SyncCallRawResultOrError<Api>
where
    Api: CallTypeApi,
{
    Success(SyncCallRawResult<Api>),
    Error(u32),
}

#[derive(Default)]
pub struct SendRawWrapper<A>
where
    A: CallTypeApi,
{
    _phantom: PhantomData<A>,
}

/// Error type returned when a transfer-execute operation signals that it failed.
pub struct TransferExecuteFailed;

impl<A> SendRawWrapper<A>
where
    A: CallTypeApi,
{
    pub fn new() -> Self {
        SendRawWrapper {
            _phantom: PhantomData,
        }
    }

    fn load_code_metadata_to_mb(
        &self,
        code_metadata: CodeMetadata,
        code_metadata_handle: RawHandle,
    ) {
        let code_metadata_bytes = code_metadata.to_byte_array();
        A::managed_type_impl().mb_overwrite(
            use_raw_handle(code_metadata_handle),
            &code_metadata_bytes[..],
        );
    }

    pub fn direct_egld<D>(&self, to: &ManagedAddress<A>, egld_value: &BigUint<A>, data: D)
    where
        D: Into<ManagedBuffer<A>>,
    {
        let empty_mb_handle: A::ManagedBufferHandle =
            use_raw_handle(const_handles::MBUF_TEMPORARY_1);
        A::managed_type_impl().mb_overwrite(empty_mb_handle.clone(), &[]);

        A::send_api_impl().transfer_value_execute(
            to.get_handle().get_raw_handle(),
            egld_value.get_handle().get_raw_handle(),
            0,
            data.into().get_handle().get_raw_handle(),
            empty_mb_handle.get_raw_handle(),
        );
    }

    pub fn direct_egld_execute(
        &self,
        to: &ManagedAddress<A>,
        egld_value: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().transfer_value_execute(
            to.get_handle().get_raw_handle(),
            egld_value.get_handle().get_raw_handle(),
            gas_limit,
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        );
    }

    pub fn transfer_esdt_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        value: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        self.transfer_esdt_nft_execute(to, token, 0, value, gas_limit, endpoint_name, arg_buffer);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn transfer_esdt_nft_execute(
        &self,
        to: &ManagedAddress<A>,
        token: &TokenIdentifier<A>,
        nonce: u64,
        egld_value: &BigUint<A>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        let mut payments: ManagedVec<A, EsdtTokenPayment<A>> = ManagedVec::new();
        payments.push(EsdtTokenPayment::new(
            token.clone(),
            nonce,
            egld_value.clone(),
        ));
        self.multi_esdt_transfer_execute(to, &payments, gas_limit, endpoint_name, arg_buffer);
    }

    pub fn multi_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        A::send_api_impl().multi_transfer_esdt_nft_execute(
            to.get_handle().get_raw_handle(),
            payments.get_handle().get_raw_handle(),
            gas_limit,
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        );
    }

    /// `multi_transfer_esdt_nft_execute` doesn't work for a single EGLD payment,
    /// so we need a different strategy in this one particular case.
    ///
    /// Returns `true` if single EGLD payment was produced.
    fn fallback_to_single_egld_if_necessary(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EgldOrEsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> bool {
        if payments.is_empty() {
            self.direct_egld_execute(to, &BigUint::zero(), gas_limit, endpoint_name, arg_buffer);

            return true;
        }

        if let Some(single_item) = payments.is_single_item() {
            if single_item.token_identifier.is_egld() {
                self.direct_egld_execute(
                    to,
                    &single_item.amount,
                    gas_limit,
                    endpoint_name,
                    arg_buffer,
                );
                return true;
            }
        }

        false
    }

    #[deprecated(
        since = "0.59.0",
        note = "Use multi_egld_or_esdt_transfer_execute_fallible instead"
    )]
    pub fn multi_egld_or_esdt_transfer_execute(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EgldOrEsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        if self.fallback_to_single_egld_if_necessary(
            to,
            payments,
            gas_limit,
            endpoint_name,
            arg_buffer,
        ) {
            return;
        }
        A::send_api_impl().multi_transfer_esdt_nft_execute(
            to.get_handle().get_raw_handle(),
            payments.get_handle().get_raw_handle(),
            gas_limit,
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        );
    }

    #[cfg(feature = "barnard")]
    pub fn multi_egld_or_esdt_transfer_execute_fallible(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EgldOrEsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), TransferExecuteFailed> {
        if payments.is_empty() {
            use crate::{api::quick_signal_error, err_msg};

            quick_signal_error::<A>(err_msg::TRANSFER_EXECUTE_REQUIRES_PAYMENT);
        }

        let ret = A::send_api_impl().multi_transfer_esdt_nft_execute_with_return(
            to.get_handle().get_raw_handle(),
            payments.get_handle().get_raw_handle(),
            gas_limit,
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        );

        if ret == 0 {
            Ok(())
        } else {
            Err(TransferExecuteFailed)
        }
    }

    #[cfg(not(feature = "barnard"))]
    #[allow(deprecated)]
    pub fn multi_egld_or_esdt_transfer_execute_fallible(
        &self,
        to: &ManagedAddress<A>,
        payments: &ManagedVec<A, EgldOrEsdtTokenPayment<A>>,
        gas_limit: u64,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> Result<(), TransferExecuteFailed> {
        self.multi_egld_or_esdt_transfer_execute(
            to,
            payments,
            gas_limit,
            endpoint_name,
            arg_buffer,
        );
        // no fallibility before Barnard
        Ok(())
    }

    pub fn async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        egld_value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ! {
        A::send_api_impl().async_call_raw(
            to.get_handle().get_raw_handle(),
            egld_value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_async_call_raw(
        &self,
        to: &ManagedAddress<A>,
        egld_value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
        success_callback: &'static str,
        error_callback: &'static str,
        gas: u64,
        extra_gas_for_callback: u64,
        serialized_callback_closure_args: &ManagedBuffer<A>,
    ) {
        A::send_api_impl().create_async_call_raw(
            to.get_handle().get_raw_handle(),
            egld_value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            success_callback,
            error_callback,
            gas,
            extra_gas_for_callback,
            serialized_callback_closure_args
                .get_handle()
                .get_raw_handle(),
        )
    }

    /// Deploys a new contract in the same shard.
    /// Unlike `async_call_raw`, the deployment is synchronous and tx execution continues afterwards.
    /// Also unlike `async_call_raw`, it uses an argument buffer to pass arguments
    /// If the deployment fails, Option::None is returned
    pub fn deploy_contract(
        &self,
        gas: u64,
        egld_value: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let new_address_handle = A::static_var_api_impl().next_handle();
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().deploy_contract(
            gas,
            egld_value.get_handle().get_raw_handle(),
            code.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            new_address_handle,
            result_handle,
        );
        unsafe {
            (
                ManagedAddress::from_raw_handle(new_address_handle),
                ManagedVec::from_raw_handle(result_handle),
            )
        }
    }

    /// Deploys a new contract in the same shard by re-using the code of an already deployed source contract.
    /// The deployment is done synchronously and the new contract's address is returned.
    /// If the deployment fails, Option::None is returned
    pub fn deploy_from_source_contract(
        &self,
        gas: u64,
        egld_value: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> (ManagedAddress<A>, ManagedVec<A, ManagedBuffer<A>>) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        let new_address_handle = A::static_var_api_impl().next_handle();
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().deploy_from_source_contract(
            gas,
            egld_value.get_handle().get_raw_handle(),
            source_contract_address.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
            new_address_handle,
            result_handle,
        );
        unsafe {
            (
                ManagedAddress::from_raw_handle(new_address_handle),
                ManagedVec::from_raw_handle(result_handle),
            )
        }
    }

    pub fn upgrade_from_source_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        egld_value: &BigUint<A>,
        source_contract_address: &ManagedAddress<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        A::send_api_impl().upgrade_from_source_contract(
            sc_address.get_handle().get_raw_handle(),
            gas,
            egld_value.get_handle().get_raw_handle(),
            source_contract_address.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
        )
    }

    /// Upgrades a child contract of the currently executing contract.
    /// The upgrade is synchronous, and the current transaction will fail if the upgrade fails.
    /// The child contract's new init function will be called with the provided arguments
    pub fn upgrade_contract(
        &self,
        sc_address: &ManagedAddress<A>,
        gas: u64,
        egld_value: &BigUint<A>,
        code: &ManagedBuffer<A>,
        code_metadata: CodeMetadata,
        arg_buffer: &ManagedArgBuffer<A>,
    ) {
        let code_metadata_handle = const_handles::MBUF_TEMPORARY_1;
        self.load_code_metadata_to_mb(code_metadata, code_metadata_handle);
        A::send_api_impl().upgrade_contract(
            sc_address.get_handle().get_raw_handle(),
            gas,
            egld_value.get_handle().get_raw_handle(),
            code.get_handle().get_raw_handle(),
            code_metadata_handle,
            arg_buffer.get_handle().get_raw_handle(),
        )
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> SyncCallRawResult<A> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_raw(
            gas,
            address.get_handle().get_raw_handle(),
            value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        let result_vec = unsafe { ManagedVec::from_raw_handle(result_handle) };
        SyncCallRawResult(result_vec)
    }

    pub fn execute_on_same_context_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_same_context_raw(
            gas,
            address.get_handle().get_raw_handle(),
            value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        unsafe { ManagedVec::from_raw_handle(result_handle) }
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_readonly_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> ManagedVec<A, ManagedBuffer<A>> {
        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_readonly_raw(
            gas,
            address.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        unsafe { ManagedVec::from_raw_handle(result_handle) }
    }

    /// Same shard, in-line execution of another contract.
    pub fn execute_on_dest_context_error_return_raw(
        &self,
        gas: u64,
        address: &ManagedAddress<A>,
        value: &BigUint<A>,
        endpoint_name: &ManagedBuffer<A>,
        arg_buffer: &ManagedArgBuffer<A>,
    ) -> SyncCallRawResultOrError<A> {
        let result_handle = A::static_var_api_impl().next_handle();
        let result_code = A::send_api_impl().execute_on_dest_context_error_return_raw(
            gas,
            address.get_handle().get_raw_handle(),
            value.get_handle().get_raw_handle(),
            endpoint_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );
        if result_code == 0 {
            let result_vec = unsafe { ManagedVec::from_raw_handle(result_handle) };
            SyncCallRawResultOrError::Success(SyncCallRawResult(result_vec))
        } else {
            SyncCallRawResultOrError::Error(result_code as u32)
        }
    }
}

impl<A> SendRawWrapper<A>
where
    A: CallTypeApi,
{
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
        let egld_value_handle = A::managed_type_impl().bi_new_zero();

        let result_handle = A::static_var_api_impl().next_handle();
        A::send_api_impl().execute_on_dest_context_raw(
            gas,
            own_address_handle.get_raw_handle(),
            egld_value_handle.get_raw_handle(),
            function_name.get_handle().get_raw_handle(),
            arg_buffer.get_handle().get_raw_handle(),
            result_handle,
        );

        self.clean_return_data();
        unsafe { ManagedVec::from_raw_handle(result_handle) }
    }

    pub fn clean_return_data(&self) {
        A::send_api_impl().clean_return_data()
    }
}
