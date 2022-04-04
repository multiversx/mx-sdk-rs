#![no_std]
#![allow(clippy::type_complexity)]

use elrond_wasm::api::ESDT_MULTI_TRANSFER_FUNC_NAME;

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
    ) {
        if token.is_egld() {
            self.send().direct_egld(&to, &payment, ManagedBuffer::new());
        } else {
            self.send().transfer_esdt_via_async_call(
                &to,
                &token,
                0,
                &payment,
                ManagedBuffer::new(),
            );
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

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_multi(&self, to: ManagedAddress) {
        let payments = self.call_value().all_esdt_transfers();
        self.send().direct_multi(&to, &payments, &[]);
    }

    fn forward_contract_call(
        &self,
        to: ManagedAddress,
        payment_token: TokenIdentifier,
        payment_amount: BigUint,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_register_promise(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        gas_limit: u64,
        extra_gas_for_callback: u64,
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(gas_limit)
            .with_extra_gas_for_callback(extra_gas_for_callback)
            .with_success_callback(b"success_callback")
            .with_error_callback(b"error_callback")
            .register_promise();
    }

    #[endpoint]
    fn success_callback(&self, #[var_args] args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(true);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[endpoint]
    fn error_callback(&self, #[var_args] args: MultiValueEncoded<ManagedBuffer>) {
        self.async_call_callback_data().set(false);
        let args_as_vec = args.into_vec_of_buffers();
        self.async_call_event_callback(&args_as_vec);
    }

    #[view]
    #[storage_mapper("async_call_callback_data")]
    fn async_call_callback_data(&self) -> SingleValueMapper<bool>;

    #[event("async_call_event_callback")]
    fn async_call_event_callback(&self, arguments: &ManagedVec<Self::Api, ManagedBuffer>);

    #[endpoint]
    #[payable("*")]
    fn forward_async_call_half_payment(
        &self,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
        endpoint_name: ManagedBuffer,
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, TokenIdentifier::egld(), payment, endpoint_name, args)
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[endpoint]
    fn forward_async_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        #[var_args] token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty();
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        Self::Api::send_api_impl().async_call_raw(
            &to,
            &BigUint::zero(),
            &ManagedBuffer::from(&b"retrieve_multi_funds_async"[..]),
            &arg_buffer,
        );
    }

    #[endpoint]
    fn forwarder_async_send_and_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        #[var_args] token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_payments = ManagedVec::new();
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
            &all_payments,
            &b"burn_and_create_retrive_async"[..],
        );
    }

    #[endpoint]
    fn forwarder_multi_transfer_via_promise(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        extra_gas_for_callback: u64,
        #[var_args] token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new_empty();
        arg_buffer.push_arg(to);
        arg_buffer.push_arg(token_payments.raw_len() / 3);

        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        if !endpoint_name.is_empty() {
            arg_buffer.push_arg_raw(endpoint_name);
        }

        Self::Api::send_api_impl().create_async_call_raw(
            &ManagedAddress::from_raw_handle(
                Self::Api::blockchain_api_impl().get_sc_address_handle(),
            ),
            &BigUint::zero(),
            &ManagedBuffer::new_from_bytes(ESDT_MULTI_TRANSFER_FUNC_NAME),
            b"success_callback",
            b"error_callback",
            self.blockchain().get_gas_left(),
            extra_gas_for_callback,
            &arg_buffer,
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
    fn callback_raw(&self, #[var_args] args: MultiValueEncoded<ManagedBuffer>) {
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            let egld_value = self.call_value().egld_value();
            if egld_value > 0 {
                let _ = self
                    .callback_payments()
                    .push(&(TokenIdentifier::egld(), 0, egld_value));
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = Self::Api::send_api_impl().execute_on_dest_context_raw(
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let one_third_gas = self.blockchain().get_gas_left() / 3;
        let half_payment = payment / 2u32;
        let arg_buffer = args.to_arg_buffer();

        let result = Self::Api::send_api_impl().execute_on_dest_context_raw(
            one_third_gas,
            &to,
            &half_payment,
            &endpoint_name,
            &arg_buffer,
        );
        self.execute_on_dest_context_result(result);

        let result = Self::Api::send_api_impl().execute_on_dest_context_raw(
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = Self::Api::send_api_impl().execute_on_dest_context_by_caller_raw(
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = Self::Api::send_api_impl().execute_on_same_context_raw(
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = Self::Api::send_api_impl().execute_on_dest_context_readonly_raw(
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
        #[var_args] args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        Self::Api::send_api_impl()
            .deploy_contract(
                self.blockchain().get_gas_left(),
                &BigUint::zero(),
                &code,
                CodeMetadata::DEFAULT,
                &args.to_arg_buffer(),
            )
            .into()
    }

    #[endpoint]
    fn deploy_from_source(
        &self,
        source_contract_address: ManagedAddress,
        #[var_args] arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> ManagedAddress {
        let (address, _) = Self::Api::send_api_impl().deploy_from_source_contract(
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.to_arg_buffer(),
        );

        address
    }

    #[endpoint]
    fn upgrade(
        &self,
        child_sc_address: &ManagedAddress,
        new_code: &ManagedBuffer,
        #[var_args] arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        Self::Api::send_api_impl().upgrade_contract(
            child_sc_address,
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            new_code,
            CodeMetadata::UPGRADEABLE,
            &arguments.to_arg_buffer(),
        );
    }

    #[endpoint]
    fn upgrade_from_source(
        &self,
        sc_address: ManagedAddress,
        source_contract_address: ManagedAddress,
        #[var_args] arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        Self::Api::send_api_impl().upgrade_from_source_contract(
            &sc_address,
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.to_arg_buffer(),
        )
    }
}
