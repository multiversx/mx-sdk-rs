#![no_std]
#![allow(clippy::type_complexity)]

elrond_wasm::imports!();

/// Test contract for investigating async calls.
/// TODO: split into modules
#[elrond_wasm::contract]
pub trait ForwarderRaw {
    #[init]
    fn init(&self) {}

    // ASYNC CALLS

    #[endpoint]
    #[payable("*")]
    fn forward_payment(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
    ) -> SendToken<Self::Api> {
        SendToken {
            api: self.raw_vm_api(),
            to,
            token,
            amount: payment,
            data: self.types().managed_buffer_empty(),
        }
    }

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_via_transf_exec(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
    ) {
        let _ = self.send().direct(&to, &token, 0, &payment, &[]);
    }

    fn forward_contract_call(
        &self,
        to: ManagedAddress,
        payment_token: TokenIdentifier,
        payment_amount: BigUint,
        endpoint_name: ManagedBuffer,
        args: ManagedVarArgs<ManagedBuffer>,
    ) -> ContractCall<Self::Api, ()> {
        self.send()
            .contract_call(to, endpoint_name)
            .add_token_transfer(payment_token, 0, payment_amount)
            .with_arguments_raw(args.to_arg_buffer())
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) -> AsyncCall {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .async_call()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call_half_payment(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) -> AsyncCall {
        let half_payment = payment / 2u32;
        self.forward_async_call(to, token, half_payment, endpoint_name, args)
    }

    #[endpoint]
    #[payable("EGLD")]
    fn forward_transf_exec_egld(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        self.forward_contract_call(
            to,
            self.types().token_identifier_egld(),
            payment,
            endpoint_name,
            args,
        )
        .with_gas_limit(self.blockchain().get_gas_left() / 2)
        .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_esdt(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[endpoint]
    fn forward_async_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        #[var_args] token_payments: ManagedVarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty(self.raw_vm_api());
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        self.raw_vm_api().async_call_raw(
            &to,
            &self.types().big_uint_zero(),
            &self
                .types()
                .managed_buffer_from(&b"retrieve_multi_funds_async"[..]),
            &arg_buffer,
        );
    }

    #[endpoint]
    fn forwarder_async_send_and_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        #[var_args] token_payments: ManagedVarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_payments = Vec::new();
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            all_payments.push(EsdtTokenPayment {
                token_identifier,
                token_nonce,
                amount,
                token_type: EsdtTokenType::Invalid, // ignored
            });
        }

        self.send().transfer_multiple_esdt_via_async_call(
            &to,
            &all_payments.managed_into(self.raw_vm_api()),
            &b"burn_and_create_retrive_async"[..],
        );
    }

    #[view]
    #[storage_mapper("callback_data")]
    fn callback_data(&self) -> VecMapper<ManagedVec<Self::Api, ManagedBuffer>>;

    #[storage_mapper("callback_payments")]
    fn callback_payments(&self) -> VecMapper<(TokenIdentifier, u64, BigUint)>;

    #[endpoint]
    fn clear_callback_info(&self) {
        self.callback_data().clear();
    }

    #[callback_raw]
    fn callback_raw(&self, #[var_args] args: ManagedVarArgs<ManagedBuffer>) {
        let payments = self.call_value().all_esdt_transfers();
        if payments.len() == 0 {
            let egld_value = self.call_value().egld_value();
            if egld_value > 0 {
                let _ = self.callback_payments().push(&(
                    self.types().token_identifier_egld(),
                    0,
                    egld_value,
                ));
            }
        } else {
            for payment in payments.into_iter() {
                let _ = self.callback_payments().push(&(
                    payment.token_identifier,
                    payment.token_nonce,
                    payment.amount,
                ));
            }
        }

        let args_as_vec = args.into_vec_of_buffers();
        self.callback_raw_event(&args_as_vec);

        let _ = self.callback_data().push(&args_as_vec);
    }

    #[event("callback_raw")]
    fn callback_raw_event(&self, arguments: &ManagedVec<Self::Api, ManagedBuffer>);

    // SYNC CALLS

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.raw_vm_api().execute_on_dest_context_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.to_arg_buffer(),
        );

        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context_twice(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;
        let half_payment = payment / 2u32;
        let arg_buffer = args.to_arg_buffer();

        let result = self.raw_vm_api().execute_on_dest_context_raw(
            one_third_gas,
            &to,
            &half_payment,
            &endpoint_name,
            &arg_buffer,
        );
        self.execute_on_dest_context_result(result);

        let result = self.raw_vm_api().execute_on_dest_context_raw(
            one_third_gas,
            &to,
            &half_payment,
            &endpoint_name,
            &arg_buffer,
        );
        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context_by_caller(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.raw_vm_api().execute_on_dest_context_by_caller_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.to_arg_buffer(),
        );

        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_same_context(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.raw_vm_api().execute_on_same_context_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.to_arg_buffer(),
        );

        self.execute_on_same_context_result(result);
    }

    #[endpoint]
    fn call_execute_on_dest_context_readonly(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.raw_vm_api().execute_on_dest_context_readonly_raw(
            half_gas,
            &to,
            &endpoint_name,
            &args.to_arg_buffer(),
        );

        self.execute_on_dest_context_result(result);
    }

    #[event("execute_on_dest_context_result")]
    fn execute_on_dest_context_result(&self, result: ManagedVec<Self::Api, ManagedBuffer>);

    #[event("execute_on_same_context_result")]
    fn execute_on_same_context_result(&self, result: ManagedVec<Self::Api, ManagedBuffer>);

    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) -> MultiResult2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.raw_vm_api()
            .deploy_contract(
                self.blockchain().get_gas_left(),
                &self.types().big_uint_zero(),
                &code,
                CodeMetadata::DEFAULT,
                &args.to_arg_buffer(),
            )
            .into()
    }
}
