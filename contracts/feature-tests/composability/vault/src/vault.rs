#![no_std]
#![allow(clippy::type_complexity)]

use multiversx_sc::codec::Empty;

multiversx_sc::imports!();

/// General test contract.
/// Used especially for investigating async calls and contract interaction in general.
#[multiversx_sc::contract]
pub trait Vault {
    #[init]
    fn init(&self, opt_arg_to_echo: OptionalValue<ManagedBuffer>) -> OptionalValue<ManagedBuffer> {
        opt_arg_to_echo
    }

    #[upgrade]
    #[label("upgrade")]
    fn upgrade(
        &self,
        opt_arg_to_echo: OptionalValue<ManagedBuffer>,
    ) -> MultiValue2<&'static str, OptionalValue<ManagedBuffer>> {
        self.upgraded_event();
        ("upgraded", opt_arg_to_echo).into()
    }

    #[event("upgraded")]
    fn upgraded_event(&self);

    #[endpoint]
    fn echo_arguments(
        &self,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        self.call_counts(ManagedBuffer::from(b"echo_arguments"))
            .update(|c| *c += 1);
        args
    }

    #[endpoint]
    fn echo_arguments_without_storage(
        &self,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        args
    }

    #[endpoint]
    fn echo_caller(&self) -> ManagedAddress {
        self.blockchain().get_caller()
    }

    fn all_transfers_multi(&self) -> MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue> {
        self.call_value()
            .all_multi_transfers()
            .clone_value()
            .into_multi_value()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds(&self) {
        let esdt_transfers_multi = self.all_transfers_multi();
        self.accept_funds_event(&self.call_value().egld_value(), &esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds"))
            .update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_echo_payment(
        &self,
    ) -> MultiValue2<BigUint, MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>> {
        let egld_value = self.call_value().egld_value();
        let esdt_transfers_multi = self.all_transfers_multi();
        self.accept_funds_event(&egld_value, &esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds_echo_payment"))
            .update(|c| *c += 1);

        (egld_value.clone_value(), esdt_transfers_multi).into()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_single_esdt_transfer(&self) {
        let _ = self.call_value().single_esdt();
    }

    #[payable("*")]
    #[endpoint]
    fn reject_funds(&self) {
        let esdt_transfers_multi = self.all_transfers_multi();
        self.reject_funds_event(&self.call_value().egld_value(), &esdt_transfers_multi);
        sc_panic!("reject_funds");
    }

    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_with_transfer_exec(
        &self,
        token: TokenIdentifier,
        amount: BigUint,
        opt_receive_func: OptionalValue<ManagedBuffer>,
    ) {
        let caller = self.blockchain().get_caller();
        let func_name = opt_receive_func.into_option().unwrap_or_default();

        self.tx()
            .to(&caller)
            .gas(50_000_000u64)
            .raw_call(func_name)
            .single_esdt(&token, 0u64, &amount)
            .transfer_execute();
    }

    #[allow_multiple_var_args]
    #[label("promises-endpoint")]
    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_promises(
        &self,
        back_transfers: OptionalValue<u64>,
        back_transfer_value: OptionalValue<BigUint>,
    ) {
        let payment = self.call_value().egld_or_single_esdt();
        let caller = self.blockchain().get_caller();
        let endpoint_name = ManagedBuffer::from(b"");
        let nr_callbacks = match back_transfers.into_option() {
            Some(nr) => nr,
            None => sc_panic!("Nr of calls is None"),
        };

        let value = match back_transfer_value.into_option() {
            Some(val) => val,
            None => sc_panic!("Value for parent callback is None"),
        };

        let return_payment =
            EgldOrEsdtTokenPayment::new(payment.token_identifier, payment.token_nonce, value);

        self.num_called_retrieve_funds_promises()
            .update(|c| *c += 1);

        for _ in 0..nr_callbacks {
            self.num_async_calls_sent_from_child().update(|c| *c += 1);

            self.tx()
                .to(&caller)
                .raw_call(endpoint_name.clone())
                .payment(&return_payment)
                .gas(self.blockchain().get_gas_left() / 2)
                .transfer_execute()
        }
    }

    #[endpoint]
    fn retrieve_funds(&self, token: EgldOrEsdtTokenIdentifier, nonce: u64, amount: BigUint) {
        self.retrieve_funds_event(&token, nonce, &amount);
        let caller = self.blockchain().get_caller();

        if let Some(esdt_token_id) = token.into_esdt_option() {
            self.tx()
                .to(caller)
                .esdt((esdt_token_id, nonce, amount))
                .transfer();
        } else {
            self.tx().to(caller).egld(amount).transfer();
        }
    }

    #[endpoint]
    #[payable("*")]
    fn retrieve_funds_egld_or_single_esdt(&self) {
        let token = self.call_value().egld_or_single_esdt();
        self.retrieve_funds_event(&token.token_identifier, token.token_nonce, &token.amount);

        if let Some(esdt_token_id) = token.token_identifier.into_esdt_option() {
            self.tx()
                .to(ToCaller)
                .esdt((esdt_token_id, token.token_nonce, token.amount))
                .transfer();
        } else {
            self.tx().to(ToCaller).egld(token.amount).transfer();
        }
    }

    #[endpoint]
    #[payable("*")]
    fn retrieve_funds_multi_esdt(&self) {
        let tokens = self.call_value().all_esdt_transfers().clone_value();

        self.tx().to(ToCaller).multi_esdt(tokens).transfer();
    }

    #[endpoint]
    fn retrieve_multi_funds_async(
        &self,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        let mut all_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_id, nonce, amount) = multi_arg.into_tuple();

            all_payments.push(EsdtTokenPayment::new(token_id, nonce, amount));
        }

        self.tx().to(caller).payment(all_payments).transfer();
    }

    #[payable("*")]
    #[endpoint]
    fn burn_and_create_retrieve_async(&self) {
        let payments = self.call_value().all_esdt_transfers();
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new());

        let mut new_tokens = ManagedVec::new();

        for payment in payments.iter() {
            // burn old tokens
            self.send().esdt_local_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            // create new ones
            let new_token_nonce = self.send().esdt_nft_create(
                &payment.token_identifier,
                &payment.amount,
                &ManagedBuffer::new(),
                &BigUint::zero(),
                &ManagedBuffer::new(),
                &Empty,
                &uris,
            );

            new_tokens.push(EsdtTokenPayment::new(
                payment.token_identifier.clone(),
                new_token_nonce,
                payment.amount.clone(),
            ));
        }

        self.tx().to(ToCaller).payment(new_tokens).transfer();
    }

    #[endpoint]
    #[payable("*")]
    fn explicit_panic(&self) {
        sc_panic!("explicit panic");
    }

    #[event("accept_funds")]
    fn accept_funds_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    );

    #[event("reject_funds")]
    fn reject_funds_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EgldOrEsdtTokenPaymentMultiValue>,
    );

    #[event("retrieve_funds")]
    fn retrieve_funds_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[endpoint]
    fn get_owner_address(&self) -> ManagedAddress {
        self.blockchain().get_owner_address()
    }

    /// We already leave a trace of the calls using the event logs;
    /// this additional counter has the role of showing that storage also gets saved correctly.
    #[view]
    #[storage_mapper("call_counts")]
    fn call_counts(&self, endpoint: ManagedBuffer) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("num_called_retrieve_funds_promises")]
    fn num_called_retrieve_funds_promises(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("num_async_calls_sent_from_child")]
    fn num_async_calls_sent_from_child(&self) -> SingleValueMapper<usize>;
}
