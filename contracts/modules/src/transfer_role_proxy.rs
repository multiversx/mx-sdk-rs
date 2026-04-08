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
        payments: &PaymentsVec<Self::Api>,
        data: ManagedBuffer,
    ) -> ! {
        let transaction = self.tx().to(&dest).raw_call(data).payment(payments);

        self.execute_async_call(original_caller, payments, transaction, None)
    }

    fn transfer_to_contract_typed_call<T>(
        &self,
        original_caller: ManagedAddress,
        transaction: Tx<
            TxScEnv<Self::Api>,
            (),
            &ManagedAddress,
            &ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>>,
            (),
            FunctionCall<Self::Api>,
            (),
        >,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> !
    where
        T: TopEncodeMulti,
    {
        self.execute_async_call(
            original_caller,
            transaction.payment,
            transaction,
            opt_custom_callback,
        );
    }

    fn transfer_to_contract_raw(
        &self,
        original_caller: ManagedAddress,
        dest: ManagedAddress,
        payments: &PaymentsVec<Self::Api>,
        endpoint_name: ManagedBuffer,
        args: ManagedArgBuffer<Self::Api>,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> ! {
        let transaction = self
            .tx()
            .to(&dest)
            .raw_call(endpoint_name)
            .payment(payments)
            .arguments_raw(args);

        self.execute_async_call(original_caller, payments, transaction, opt_custom_callback)
    }

    fn execute_async_call(
        &self,
        original_caller: ManagedAddress,
        initial_payments: &PaymentsVec<Self::Api>,
        transaction: Tx<
            TxScEnv<Self::Api>,
            (),
            &ManagedAddress,
            &ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>>,
            (),
            FunctionCall<Self::Api>,
            (),
        >,
        opt_custom_callback: Option<CallbackClosure<Self::Api>>,
    ) -> ! {
        require!(
            self.destination_whitelist().contains(transaction.to),
            "Destination address not whitelisted"
        );

        let remaining_gas = self.blockchain().get_gas_left();
        let cb_gas_needed = CALLBACK_RESERVED_GAS_PER_TOKEN * transaction.payment.len() as u64;
        require!(
            remaining_gas > cb_gas_needed,
            "Not enough gas to launch async call"
        );

        let cb = match opt_custom_callback {
            Some(custom_cb) => custom_cb,
            None => TransferRoleProxyModule::callbacks(self)
                .transfer_callback(original_caller, initial_payments.clone()),
        };

        transaction.callback(cb).async_call_and_exit()
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
                    self.tx()
                        .to(&original_caller)
                        .payment(initial_payments)
                        .transfer();
                }

                let mut err_result = MultiValueEncoded::new();
                err_result.push(ManagedBuffer::new_from_bytes(ERR_CALLBACK_MSG));
                err_result.push(err.err_msg.clone());

                sc_print!("{}", err.err_msg);

                err_result
            }
        }
    }

    #[storage_mapper("transfer_role_proxy:destination_whitelist")]
    fn destination_whitelist(&self) -> UnorderedSetMapper<ManagedAddress>;
}
