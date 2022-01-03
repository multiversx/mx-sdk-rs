#![no_std]
#![allow(clippy::type_complexity)]

elrond_wasm::imports!();

/// General test contract.
/// Used especially for investigating async calls and contract interaction in general.
#[elrond_wasm::contract]
pub trait Vault {
    #[init]
    fn init(
        &self,
        #[var_args] opt_arg_to_echo: OptionalArg<ManagedBuffer>,
    ) -> OptionalResult<ManagedBuffer> {
        opt_arg_to_echo
    }

    #[payable("*")]
    #[endpoint]
    fn just_accept_funds(&self) {
        self.call_counts(b"accept_funds").update(|c| *c += 1);
    }

    #[endpoint]
    fn echo_arguments(
        &self,
        #[var_args] args: ManagedVarArgs<ManagedBuffer>,
    ) -> SCResult<ManagedMultiResultVec<ManagedBuffer>> {
        self.call_counts(b"echo_arguments").update(|c| *c += 1);
        Ok(args)
    }

    #[endpoint]
    fn echo_caller(&self) -> ManagedAddress {
        self.blockchain().get_caller()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] payment: BigUint,
    ) {
        let token_type = self.call_value().esdt_token_type();

        self.accept_funds_event(&token, token_type.as_type_name(), &payment, nonce);

        self.call_counts(b"accept_funds").update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_multi_transfer(&self) {
        let payments = self.call_value().all_esdt_transfers();

        for payment in payments.into_iter() {
            self.accept_funds_event(
                &payment.token_identifier,
                payment.token_type.as_type_name(),
                &payment.amount,
                payment.token_nonce,
            );
        }

        self.call_counts(b"accept_funds_multi_transfer")
            .update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_multi_funds_echo(
        &self,
    ) -> ManagedMultiResultVec<MultiArg3<TokenIdentifier, u64, BigUint>> {
        let payments = self.call_value().all_esdt_transfers();
        let mut result = ManagedMultiResultVec::new();

        for payment in payments.into_iter() {
            result.push(
                (
                    payment.token_identifier,
                    payment.token_nonce,
                    payment.amount,
                )
                    .into(),
            );
        }

        result
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_echo_payment(
        &self,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment_amount] token_payment: BigUint,
        #[payment_nonce] token_nonce: u64,
    ) -> SCResult<MultiResult4<TokenIdentifier, ManagedBuffer, BigUint, u64>> {
        let token_type = self.call_value().esdt_token_type();

        self.accept_funds_event(
            &token_identifier,
            token_type.as_type_name(),
            &token_payment,
            token_nonce,
        );

        self.call_counts(b"accept_funds_echo_payment")
            .update(|c| *c += 1);

        Ok((
            token_identifier,
            token_type.as_type_name().into(),
            token_payment,
            token_nonce,
        )
            .into())
    }

    #[payable("*")]
    #[endpoint]
    fn reject_funds(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment] payment: BigUint,
    ) -> SCResult<()> {
        self.reject_funds_event(&token, &payment);
        sc_error!("reject_funds")
    }

    #[endpoint]
    fn retrieve_funds(
        &self,
        token: TokenIdentifier,
        nonce: u64,
        amount: BigUint,
        #[var_args] return_message: OptionalArg<ManagedBuffer>,
    ) {
        self.retrieve_funds_event(&token, nonce, &amount);

        let caller = self.blockchain().get_caller();
        let data = match return_message {
            OptionalArg::Some(data) => data,
            OptionalArg::None => ManagedBuffer::new(),
        };

        if token.is_egld() {
            self.send().direct_egld(&caller, &amount, data);
        } else {
            self.send()
                .transfer_esdt_via_async_call(&caller, &token, nonce, &amount, data);
        }
    }

    #[endpoint]
    fn retrieve_multi_funds_async(
        &self,
        #[var_args] token_payments: ManagedVarArgs<MultiArg3<TokenIdentifier, u64, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        let mut all_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_id, nonce, amount) = multi_arg.into_tuple();

            all_payments.push(EsdtTokenPayment {
                token_identifier: token_id,
                token_nonce: nonce,
                amount,
                token_type: EsdtTokenType::Invalid,
            });
        }

        self.send()
            .transfer_multiple_esdt_via_async_call(&caller, &all_payments, b"");
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
                &(),
                &uris,
            );

            new_tokens.push(EsdtTokenPayment {
                token_identifier: payment.token_identifier,
                token_nonce: new_token_nonce,
                amount: payment.amount,
                token_type: EsdtTokenType::Invalid, // ignored
            });
        }

        self.send().transfer_multiple_esdt_via_async_call(
            &self.blockchain().get_caller(),
            &new_tokens,
            &[],
        );
    }

    /// TODO: invert token_payment and token_nonce, for consistency.
    #[event("accept_funds")]
    fn accept_funds_event(
        &self,
        #[indexed] token_identifier: &TokenIdentifier,
        #[indexed] token_type: &[u8],
        #[indexed] token_payment: &BigUint,
        #[indexed] token_nonce: u64,
    );

    #[event("reject_funds")]
    fn reject_funds_event(&self, #[indexed] token: &TokenIdentifier, #[indexed] payment: &BigUint);

    #[event("retrieve_funds")]
    fn retrieve_funds_event(
        &self,
        #[indexed] token: &TokenIdentifier,
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
    fn call_counts(&self, endpoint: &[u8]) -> SingleValueMapper<usize>;
}
