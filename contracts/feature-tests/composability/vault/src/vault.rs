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

    fn all_transfers_multi(&self) -> MultiValueEncoded<PaymentMultiValue> {
        self.call_value().all().clone().into_multi_value()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds(&self) {
        let esdt_transfers_multi = self.all_transfers_multi();
        self.accept_funds_event(&esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds"))
            .update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_echo_payment(&self) -> MultiValueEncoded<PaymentMultiValue> {
        let esdt_transfers_multi = self.all_transfers_multi();
        self.accept_funds_event(&esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds_echo_payment"))
            .update(|c| *c += 1);

        esdt_transfers_multi
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
        self.reject_funds_event(&esdt_transfers_multi);
        sc_panic!("reject_funds");
    }

    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_with_transfer_exec(
        &self,
        token: EsdtTokenIdentifier,
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

    #[endpoint]
    fn retrieve_funds(&self, token: EgldOrEsdtTokenIdentifier, nonce: u64, amount: NonZeroBigUint) {
        self.retrieve_funds_event(&token, nonce, amount.as_big_uint());
        let caller = self.blockchain().get_caller();

        if let Some(esdt_token_id) = token.into_esdt_option() {
            self.tx()
                .to(caller)
                .payment(Payment::new(esdt_token_id.into(), nonce, amount))
                .transfer();
        } else {
            self.tx().to(caller).egld(amount.as_big_uint()).transfer();
        }
    }

    #[endpoint]
    #[payable("*")]
    fn retrieve_funds_egld_or_single_esdt(&self) {
        if let Some(payment) = self.call_value().single_optional() {
            self.retrieve_funds_event(
                payment.token_identifier.as_legacy(),
                payment.token_nonce,
                payment.amount.as_big_uint(),
            );

            self.tx().to(ToCaller).payment(payment).transfer();
        }
    }

    #[endpoint]
    #[payable("*")]
    fn retrieve_received_funds_immediately(&self) {
        let tokens = self.call_value().all();

        self.tx().to(ToCaller).payment(tokens).transfer();
    }

    #[endpoint]
    fn retrieve_funds_multi(&self, transfers: MultiValueEncoded<PaymentMultiValue>) {
        self.retrieve_funds_multi_event(&transfers);

        self.tx()
            .to(ToCaller)
            .payment(transfers.convert_payment())
            .transfer();
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
    fn accept_funds_event(&self, #[indexed] multi_esdt: &MultiValueEncoded<PaymentMultiValue>);

    #[event("reject_funds")]
    fn reject_funds_event(&self, #[indexed] multi_esdt: &MultiValueEncoded<PaymentMultiValue>);

    #[event("retrieve_funds")]
    fn retrieve_funds_event(
        &self,
        #[indexed] token: &EgldOrEsdtTokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[event("retrieve_funds_multi")]
    fn retrieve_funds_multi_event(
        &self,
        #[indexed] transfers: &MultiValueEncoded<PaymentMultiValue>,
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
}
