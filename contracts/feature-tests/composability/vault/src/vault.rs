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

    fn esdt_transfers_multi(&self) -> MultiValueEncoded<EsdtTokenPaymentMultiValue> {
        self.call_value().all_esdt_transfers().into_multi_value()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds(&self) {
        let esdt_transfers_multi = self.esdt_transfers_multi();
        self.accept_funds_event(&self.call_value().egld_value(), &esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds"))
            .update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_echo_payment(
        &self,
    ) -> MultiValue2<BigUint, MultiValueEncoded<EsdtTokenPaymentMultiValue>> {
        let egld_value = self.call_value().egld_value();
        let esdt_transfers_multi = self.esdt_transfers_multi();
        self.accept_funds_event(&egld_value, &esdt_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds_echo_payment"))
            .update(|c| *c += 1);

        (egld_value, esdt_transfers_multi).into()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_single_esdt_transfer(&self) {
        let _ = self.call_value().single_esdt();
    }

    #[payable("*")]
    #[endpoint]
    fn reject_funds(&self) {
        let esdt_transfers_multi = self.esdt_transfers_multi();
        self.reject_funds_event(&self.call_value().egld_value(), &esdt_transfers_multi);
        sc_panic!("reject_funds");
    }

    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_with_transfer_exec(
        &self,
        #[payment_multi] _payments: ManagedVec<EsdtTokenPayment<Self::Api>>,
        token: TokenIdentifier,
        amount: BigUint,
        opt_receive_func: OptionalValue<ManagedBuffer>,
    ) {
        let caller = self.blockchain().get_caller();
        let func_name = opt_receive_func.into_option().unwrap_or_default();

        self.send_raw()
            .transfer_esdt_execute(
                &caller,
                &token,
                &amount,
                50_000_000,
                &func_name,
                &ManagedArgBuffer::new(),
            )
            .unwrap_or_else(|_| sc_panic!("ESDT transfer failed"));
    }

    #[endpoint]
    fn retrieve_funds(&self, token: EgldOrEsdtTokenIdentifier, nonce: u64, amount: BigUint) {
        self.retrieve_funds_event(&token, nonce, &amount);
        let caller = self.blockchain().get_caller();

        if let Some(esdt_token_id) = token.into_esdt_option() {
            self.send()
                .transfer_esdt_via_async_call(caller, esdt_token_id, nonce, amount);
        } else {
            self.send().direct_egld(&caller, &amount);
        }
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

        self.send()
            .transfer_multiple_esdt_via_async_call(caller, all_payments);
    }

    #[payable("*")]
    #[endpoint]
    fn burn_and_create_retrive_async(&self) {
        let payments = self.call_value().all_esdt_transfers();
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new());

        let mut new_tokens = ManagedVec::new();

        for payment in payments.into_iter() {
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
                payment.token_identifier,
                new_token_nonce,
                payment.amount,
            ));
        }

        self.send()
            .transfer_multiple_esdt_via_async_call(self.blockchain().get_caller(), new_tokens);
    }

    /// TODO: invert token_payment and token_nonce, for consistency.
    #[event("accept_funds")]
    fn accept_funds_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
    );

    #[event("reject_funds")]
    fn reject_funds_event(
        &self,
        #[indexed] egld_value: &BigUint,
        #[indexed] multi_esdt: &MultiValueEncoded<EsdtTokenPaymentMultiValue>,
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
}
