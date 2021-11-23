#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const EGLD_NUM_DECIMALS: usize = 18;

/// Converts between EGLD and a wrapped EGLD ESDT token.
///	1 EGLD = 1 wrapped EGLD and is interchangeable at all times.
/// Also manages the supply of wrapped EGLD tokens.
#[elrond_wasm::contract]
pub trait EgldEsdtSwap {
    #[init]
    fn init(&self) {}

    // endpoints - owner-only

    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueWrappedEgld)]
    fn issue_wrapped_egld(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
        #[payment] issue_cost: BigUint,
    ) -> SCResult<AsyncCall> {
        require!(
            self.wrapped_egld_token_id().is_empty(),
            "wrapped egld was already issued"
        );

        let caller = self.blockchain().get_caller();

        self.issue_started_event(&caller, &token_ticker, &initial_supply);

        Ok(self
            .send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: EGLD_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller)))
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] returned_tokens: BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.unused_wrapped_egld().set(&returned_tokens);
                self.wrapped_egld_token_id().set(&token_identifier);
            },
            ManagedAsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, &message.err_msg);

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[only_owner]
    #[endpoint(mintWrappedEgld)]
    fn mint_wrapped_egld(&self, amount: BigUint) -> SCResult<AsyncCall> {
        require!(
            !self.wrapped_egld_token_id().is_empty(),
            "Wrapped EGLD was not issued yet"
        );

        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
        let caller = self.blockchain().get_caller();
        self.mint_started_event(&caller, &amount);

        Ok(self
            .send()
            .esdt_system_sc_proxy()
            .mint(&wrapped_egld_token_id, &amount)
            .async_call()
            .with_callback(self.callbacks().esdt_mint_callback(&caller, &amount)))
    }

    #[callback]
    fn esdt_mint_callback(
        &self,
        caller: &ManagedAddress,
        amount: &BigUint,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.mint_success_event(caller);
                self.unused_wrapped_egld()
                    .update(|unused_wrapped_egld| *unused_wrapped_egld += amount);
            },
            ManagedAsyncCallResult::Err(message) => {
                self.mint_failure_event(caller, &message.err_msg);
            },
        }
    }

    // endpoints

    #[payable("EGLD")]
    #[endpoint(wrapEgld)]
    fn wrap_egld(&self, #[payment] payment: BigUint) -> SCResult<()> {
        require!(payment > 0, "Payment must be more than 0");
        require!(
            !self.wrapped_egld_token_id().is_empty(),
            "Wrapped EGLD was not issued yet"
        );

        self.unused_wrapped_egld().update(|unused_wrapped_egld| {
            require!(
                *unused_wrapped_egld > payment,
                "Contract does not have enough wrapped EGLD. Please try again once more is minted."
            );

            *unused_wrapped_egld -= &payment;

            Ok(())
        })?;

        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &self.wrapped_egld_token_id().get(),
            0,
            &payment,
            b"wrapping",
        );

        self.wrap_egld_event(&caller, &payment);

        Ok(())
    }

    #[payable("*")]
    #[endpoint(unwrapEgld)]
    fn unwrap_egld(
        &self,
        #[payment] wrapped_egld_payment: BigUint,
        #[payment_token] token_identifier: TokenIdentifier,
    ) -> SCResult<()> {
        require!(
            !self.wrapped_egld_token_id().is_empty(),
            "Wrapped EGLD was not issued yet"
        );

        let wrapped_egld_token_identifier = self.wrapped_egld_token_id().get();
        require!(
            token_identifier == wrapped_egld_token_identifier,
            "Wrong esdt token"
        );

        require!(wrapped_egld_payment > 0, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            wrapped_egld_payment
                <= self
                    .blockchain()
                    .get_sc_balance(&TokenIdentifier::egld(), 0),
            "Contract does not have enough funds"
        );

        self.unused_wrapped_egld()
            .update(|unused_wrapped_egld| *unused_wrapped_egld += &wrapped_egld_payment);

        // 1 wrapped EGLD = 1 EGLD, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_egld(&caller, &wrapped_egld_payment, b"unwrapping");

        self.unwrap_egld_event(&caller, &wrapped_egld_payment);

        Ok(())
    }

    #[view(getLockedEgldBalance)]
    fn get_locked_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
    }

    // storage

    #[view(getWrappedEgldTokenIdentifier)]
    #[storage_mapper("wrapped_egld_token_id")]
    fn wrapped_egld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUnusedWrappedEgld)]
    #[storage_mapper("unused_wrapped_egld")]
    fn unused_wrapped_egld(&self) -> SingleValueMapper<BigUint>;

    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_ticker: &ManagedBuffer,
        initial_supply: &BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

    #[event("mint-started")]
    fn mint_started_event(&self, #[indexed] caller: &ManagedAddress, amount: &BigUint);

    #[event("mint-success")]
    fn mint_success_event(&self, #[indexed] caller: &ManagedAddress);

    #[event("mint-failure")]
    fn mint_failure_event(&self, #[indexed] caller: &ManagedAddress, message: &ManagedBuffer);

    #[event("wrap-egld")]
    fn wrap_egld_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("unwrap-egld")]
    fn unwrap_egld_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
