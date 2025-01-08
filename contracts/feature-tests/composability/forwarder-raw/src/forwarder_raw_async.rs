multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ForwarderRawAsync: super::forwarder_raw_common::ForwarderRawCommon {
    #[endpoint]
    #[payable("*")]
    fn forward_payment(&self, to: ManagedAddress) {
        let (token, payment) = self.call_value().egld_or_single_fungible_esdt();
        self.tx()
            .to(to)
            .egld_or_single_esdt(&token, 0, &payment)
            .transfer();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_via_transf_exec(&self, to: ManagedAddress) {
        let (token, payment) = self.call_value().single_fungible_esdt();
        self.tx()
            .to(&to)
            .single_esdt(&token, 0, &payment)
            .transfer();
    }

    #[endpoint]
    #[payable("*")]
    fn forward_direct_esdt_multi(&self, to: ManagedAddress) {
        let payments = self.call_value().all_esdt_transfers();
        self.tx().to(&to).payment(payments).transfer();
    }

    fn forward_contract_call(
        &self,
        to: ManagedAddress,
        payment_token: EgldOrEsdtTokenIdentifier,
        payment_amount: BigUint,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> Tx<
        TxScEnv<Self::Api>,
        (),
        ManagedAddress,
        EgldOrEsdtTokenPayment<Self::Api>,
        (),
        FunctionCall<Self::Api>,
        (),
    > {
        self.tx()
            .to(to)
            .raw_call(endpoint_name)
            .arguments_raw(args.to_arg_buffer())
            .payment(EgldOrEsdtTokenPayment::new(
                payment_token,
                0,
                payment_amount,
            ))
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
            .async_call_and_exit()
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
            .async_call_and_exit()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn forward_transf_exec_egld(
        &self,
        to: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let payment = self.call_value().egld();
        self.forward_contract_call(
            to,
            EgldOrEsdtTokenIdentifier::egld(),
            payment.clone(),
            endpoint_name,
            args,
        )
        .gas(self.blockchain().get_gas_left() / 2)
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
            EgldOrEsdtTokenIdentifier::esdt(token.clone()),
            payment.clone(),
            endpoint_name,
            args,
        )
        .gas(self.blockchain().get_gas_left() / 2)
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
            .gas(self.blockchain().get_gas_left() / 2)
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
        .gas(self.blockchain().get_gas_left() / 2)
        .transfer_execute();
        self.forward_contract_call(to, token, half_payment, endpoint_name, args)
            .gas(self.blockchain().get_gas_left() / 2)
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

        self.tx()
            .to(&to)
            .raw_call("retrieve_multi_funds_async")
            .arguments_raw(arg_buffer)
            .async_call_and_exit();
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

        self.tx()
            .raw_call("burn_and_create_retrieve_async")
            .to(&to)
            .payment(&all_payments)
            .async_call_and_exit()
    }
}
