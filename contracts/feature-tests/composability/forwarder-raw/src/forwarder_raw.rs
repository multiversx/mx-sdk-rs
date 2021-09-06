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
    ) -> SendToken<Self::SendApi> {
        SendToken {
            api: self.send(),
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
        args: VarArgs<ManagedBuffer>,
    ) -> ContractCall<Self::SendApi, ()> {
        let mut contract_call = ContractCall::new(self.send(), to, endpoint_name)
            .add_token_transfer(payment_token, 0, payment_amount);
        for arg in args.into_vec() {
            contract_call.push_endpoint_arg(arg);
        }
        contract_call
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) -> AsyncCall<Self::SendApi> {
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
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) -> AsyncCall<Self::SendApi> {
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
        #[var_args] args: VarArgs<ManagedBuffer>,
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
        #[var_args] args: VarArgs<ManagedBuffer>,
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
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[view]
    #[storage_mapper("callback_data")]
    fn callback_data(
        &self,
    ) -> VecMapper<Self::Storage, (TokenIdentifier, BigUint, Vec<ManagedBuffer>)>;

    #[view]
    fn callback_data_at_index(
        &self,
        index: usize,
    ) -> MultiResult3<TokenIdentifier, BigUint, MultiResultVec<ManagedBuffer>> {
        let (token, payment, args) = self.callback_data().get(index);
        (token, payment, args.into()).into()
    }

    #[endpoint]
    fn clear_callback_info(&self) {
        self.callback_data().clear();
    }

    #[callback_raw]
    fn callback_raw(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        let args_vec = args.into_vec();
        self.callback_raw_event(&token, &payment, args_vec.as_slice().to_vec());

        let _ = self.callback_data().push(&(token, payment, args_vec));
    }

    #[event("callback_raw")]
    fn callback_raw_event(
        &self,
        #[indexed] token: &TokenIdentifier,
        #[indexed] payment: &BigUint,
        arguments: Vec<ManagedBuffer>,
    );

    // SYNC CALLS

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context(
        &self,
        to: ManagedAddress,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send().execute_on_dest_context_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.into_vec().managed_into(self.type_manager()),
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
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;
        let half_payment = payment / 2u32;
        let arg_buffer = args.into_vec().managed_into(self.type_manager());

        let result = self.send().execute_on_dest_context_raw(
            one_third_gas,
            &to,
            &half_payment,
            &endpoint_name,
            &arg_buffer,
        );
        self.execute_on_dest_context_result(result);

        let result = self.send().execute_on_dest_context_raw(
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
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send().execute_on_dest_context_by_caller_raw(
            half_gas,
            &to,
            &payment,
            &endpoint_name,
            &args.into_vec().managed_into(self.type_manager()),
        );

        self.execute_on_dest_context_result(result);
    }

    #[endpoint]
    fn call_execute_on_dest_context_readonly(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send().execute_on_dest_context_readonly_raw(
            half_gas,
            &to,
            &endpoint_name,
            &args.into_vec().managed_into(self.type_manager()),
        );

        self.execute_on_dest_context_result(result);
    }

    #[event("execute_on_dest_context_result")]
    fn execute_on_dest_context_result(&self, result: ManagedVec<Self::TypeManager, ManagedBuffer>);

    #[endpoint]
    fn deploy_contract(
        &self,
        code: ManagedBuffer,
        #[var_args] args: VarArgs<ManagedBuffer>,
    ) -> SCResult<ManagedAddress> {
        let deployed_contract_address = self
            .send()
            .deploy_contract(
                self.blockchain().get_gas_left(),
                &self.types().big_uint_zero(),
                &code,
                CodeMetadata::DEFAULT,
                &args.into_vec().managed_into(self.type_manager()),
            )
            .ok_or("Deploy failed")?;

        Ok(deployed_contract_address)
    }
}
