use multiversx_sc::codec::TopEncodeMulti;

multiversx_sc::imports!();

const CALLBACK_RESERVED_GAS_PER_TOKEN: u64 = 1_000_000;
static ERR_CALLBACK_MSG: &[u8] = b"Error received in callback:";

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;

#[multiversx_sc::module]
pub trait TransferRoleProxyModule {
    fn transfer_to_user(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: PaymentsVec<Self::Api>,
        data: ManagedBuffer,
    ) -> ! {
        let contract_call =
            ContractCallWithMultiEsdt::<Self::Api, ()>::new(dest, data, payments.clone());

        self.execute_async_call(original_caller, payments, contract_call, None);
    }

    fn transfer_to_contract_typed_call<T>(
        &self,
        original_caller: ManagedAddress,
        contract_call: ContractCallWithMultiEsdt<Self::Api, T>,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> !
    where
        T: TopEncodeMulti,
    {
        self.execute_async_call(
            original_caller,
            contract_call.esdt_payments.clone(),
            contract_call,
            opt_custom_callback,
        );
    }

    fn transfer_to_contract_raw(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: PaymentsVec<Self::Api>,
        endpoint_name: ManagedBuffer,
        args: ManagedArgBuffer<Self::Api>,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> ! {
        let contract_call =
            ContractCallWithMultiEsdt::<Self::Api, ()>::new(dest, endpoint_name, payments.clone())
                .with_raw_arguments(args);

        self.execute_async_call(
            original_caller,
            payments,
            contract_call,
            opt_custom_callback,
        );
    }

    fn execute_async_call<T>(
        &self,
        original_caller: ManagedAddress,
        initial_payments: PaymentsVec<Self::Api>,
        contract_call: ContractCallWithMultiEsdt<Self::Api, T>,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> !
    where
        T: TopEncodeMulti,
    {
        require!(
            self.destination_whitelist()
                .contains(&contract_call.basic.to),
            "Destination address not whitelisted"
        );

        let remaining_gas = self.blockchain().get_gas_left();
        let cb_gas_needed =
            CALLBACK_RESERVED_GAS_PER_TOKEN * contract_call.esdt_payments.len() as u64;
        require!(
            remaining_gas > cb_gas_needed,
            "Not enough gas to launch async call"
        );

        let async_call_gas = remaining_gas - cb_gas_needed;
        let cb = match opt_custom_callback {
            Some(custom_cb) => custom_cb,
            None => TransferRoleProxyModule::callbacks(self)
                .transfer_callback(original_caller, initial_payments),
        };

        contract_call
            .with_gas_limit(async_call_gas)
            .async_call()
            .with_callback(cb)
            .call_and_exit()
    }

    #[callback]
    fn transfer_callback(
        &self,
        original_caller: ManagedAddress,
        initial_payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        #[call_result] result: ManagedAsyncCallResult<MultiValueEncoded<ManagedBuffer>>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        match result {
            ManagedAsyncCallResult::Ok(return_values) => return_values,
            ManagedAsyncCallResult::Err(err) => {
                if !initial_payments.is_empty() {
                    self.send()
                        .direct_multi(&original_caller, &initial_payments);
                }

                let mut err_result = MultiValueEncoded::new();
                err_result.push(ManagedBuffer::new_from_bytes(ERR_CALLBACK_MSG));
                err_result.push(err.err_msg.clone());

                sc_print!("{}", err.err_msg);

                err_result
            },
        }
    }

    #[storage_mapper("transfer_role_proxy:destination_whitelist")]
    fn destination_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;
}
