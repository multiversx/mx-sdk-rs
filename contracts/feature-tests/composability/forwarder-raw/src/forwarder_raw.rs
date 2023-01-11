#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();

/// Test contract for investigating async calls.
/// TODO: split into modules
#[multiversx_sc::contract]
pub trait ForwarderRaw {
    #[init]
    fn init(&self) {}

    // ASYNC CALLS

    #[endpoint]
    #[payable("*")]
    fn forward_payment(&self, to: ManagedAddress) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        self.send().direct(&to, &token, 0, &payment);
    }

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_via_transf_exec(&self, to: ManagedAddress) {
        let (token, payment) = self.call_value().single_fungible_esdt();
        self.send().direct_esdt(&to, &token, 0, &payment);
    }

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_multi(&self, to: ManagedAddress) {
        let payments = self.call_value().all_esdt_transfers();
        self.send().direct_multi(&to, &payments);
    }

    fn forward_contract_call(
        &self,
        to: ManagedAddress,
        payment_token: EgldOrEsdtTokenIdentifier,
        payment_amount: BigUint,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> ContractCallWithEgldOrSingleEsdt<Self::Api, ()> {
        self.send()
            .contract_call(to, endpoint_name)
            .with_raw_arguments(args.to_arg_buffer())
            .with_egld_or_single_esdt_transfer((payment_token, 0, payment_amount))
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    #[payable("*")]
    fn forward_async_call_half_payment(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        let half_payment = payment / 2u32;
        self.forward_contract_call(to, token, half_payment, endpoint_name, args)
            .async_call()
            .call_and_exit()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn forward_transf_exec_egld(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld_value();
        self.forward_contract_call(
            to,
            EgldOrEsdtTokenIdentifier::egld(),
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
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (token, payment) = self.call_value().single_fungible_esdt();
        self.forward_contract_call(
            to,
            EgldOrEsdtTokenIdentifier::esdt(token),
            payment,
            endpoint_name,
            args,
        )
        .with_gas_limit(self.blockchain().get_gas_left() / 2)
        .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        self.forward_contract_call(to, token, payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_transf_exec_twice(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        let half_payment = payment / 2u32;
        self.forward_contract_call(
            to.clone(),
            token.clone(),
            half_payment.clone(),
            endpoint_name.clone(),
            args.clone(),
        )
        .with_gas_limit(self.blockchain().get_gas_left() / 2)
        .transfer_execute();
        self.forward_contract_call(to, token, half_payment, endpoint_name, args)
            .with_gas_limit(self.blockchain().get_gas_left() / 2)
            .transfer_execute();
    }

    #[endpoint]
    fn forward_async_retrieve_multi_transfer_funds(
        &self,
        to: ManagedAddress,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut arg_buffer = ManagedArgBuffer::new();
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            arg_buffer.push_arg(token_identifier);
            arg_buffer.push_arg(token_nonce);
            arg_buffer.push_arg(amount);
        }

        self.send_raw().async_call_raw(
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
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let mut all_payments = ManagedVec::new();
        for multi_arg in token_payments.into_iter() {
            let (token_identifier, token_nonce, amount) = multi_arg.into_tuple();

            all_payments.push(EsdtTokenPayment::new(token_identifier, token_nonce, amount));
        }

        ContractCallWithMultiEsdt::<Self::Api, ()>::new(
            to,
            "burn_and_create_retrive_async",
            all_payments,
        )
        .async_call()
        .call_and_exit_ignore_callback()
    }

    #[view]
    #[storage_mapper("callback_args")]
    fn callback_args(&self) -> VecMapper<ManagedVec<Self::Api, ManagedBuffer>>;

    #[view]
    #[storage_mapper("callback_payments")]
    fn callback_payments(&self) -> VecMapper<(EgldOrEsdtTokenIdentifier, u64, BigUint)>;

    #[view]
    fn callback_payments_triples(
        &self,
    ) -> MultiValueEncoded<MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint>> {
        let mut result = MultiValueEncoded::new();
        for payment_tuple in self.callback_payments().iter() {
            result.push(payment_tuple.into());
        }
        result
    }

    #[endpoint]
    fn clear_callback_info(&self) {
        self.callback_args().clear();
        self.callback_payments().clear();
    }

    /// Used in the elrond-go tests, do not remove.
    #[view]
    fn callback_args_at_index(&self, index: usize) -> MultiValueEncoded<ManagedBuffer> {
        let cb_args = self.callback_args().get(index);
        cb_args.into()
    }

    /// Used in the elrond-go tests, do not remove.
    #[view]
    fn callback_payment_at_index(
        &self,
        index: usize,
    ) -> MultiValue3<EgldOrEsdtTokenIdentifier, u64, BigUint> {
        self.callback_payments().get(index).into()
    }

    #[callback_raw]
    fn callback_raw(&self, args: MultiValueEncoded<ManagedBuffer>) {
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            let egld_value = self.call_value().egld_value();
            if egld_value > 0 {
                let _ = self.callback_payments().push(&(
                    EgldOrEsdtTokenIdentifier::egld(),
                    0,
                    egld_value,
                ));
            }
        } else {
            for payment in payments.into_iter() {
                let _ = self.callback_payments().push(&(
                    EgldOrEsdtTokenIdentifier::esdt(payment.token_identifier),
                    payment.token_nonce,
                    payment.amount,
                ));
            }
        }

        let args_as_vec = args.into_vec_of_buffers();
        self.callback_raw_event(&args_as_vec);

        let _ = self.callback_args().push(&args_as_vec);
    }

    #[event("callback_raw")]
    fn callback_raw_event(&self, arguments: &ManagedVec<Self::Api, ManagedBuffer>);

    // SYNC CALLS

    #[endpoint]
    #[payable("EGLD")]
    fn call_execute_on_dest_context(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld_value();
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send_raw().execute_on_dest_context_raw(
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
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld_value();
        let one_third_gas = self.blockchain().get_gas_left() / 3;
        let half_payment = payment / 2u32;
        let arg_buffer = args.to_arg_buffer();

        let result = self.send_raw().execute_on_dest_context_raw(
            one_third_gas,
            &to,
            &half_payment,
            &endpoint_name,
            &arg_buffer,
        );
        self.execute_on_dest_context_result(result);

        let result = self.send_raw().execute_on_dest_context_raw(
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
    fn call_execute_on_same_context(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld_value();
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send_raw().execute_on_same_context_raw(
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
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let half_gas = self.blockchain().get_gas_left() / 2;
        let result = self.send_raw().execute_on_dest_context_readonly_raw(
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
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValue2<ManagedAddress, ManagedVec<Self::Api, ManagedBuffer>> {
        self.send_raw()
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
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) -> ManagedAddress {
        let (address, _) = self.send_raw().deploy_from_source_contract(
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
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send_raw().upgrade_contract(
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
        arguments: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.send_raw().upgrade_from_source_contract(
            &sc_address,
            self.blockchain().get_gas_left(),
            &BigUint::zero(),
            &source_contract_address,
            CodeMetadata::DEFAULT,
            &arguments.to_arg_buffer(),
        )
    }
}
