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
    ) -> AsyncCall {
        require!(
            self.wrapped_egld_token_id().is_empty(),
            "wrapped egld was already issued"
        );

        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        let initial_supply = BigUint::zero();

        self.issue_started_event(&caller, &token_ticker, &initial_supply);

        self.send()
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
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
    }

    #[callback]
    fn esdt_issue_callback(
        &self,
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();

        // callback is called with ESDTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
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
    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self) -> AsyncCall {
        require!(
            !self.wrapped_egld_token_id().is_empty(),
            "Must issue token first"
        );

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.wrapped_egld_token_id().get(),
                [EsdtLocalRole::Mint, EsdtLocalRole::Burn][..]
                    .iter()
                    .cloned(),
            )
            .async_call()
    }

    // endpoints

    #[payable("EGLD")]
    #[endpoint(wrapEgld)]
    fn wrap_egld(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        require!(payment_token.is_egld(), "Only EGLD accepted");
        require!(payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
        self.send()
            .esdt_local_mint(&wrapped_egld_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct(&caller, &wrapped_egld_token_id, 0, &payment_amount, &[]);
    }

    #[payable("*")]
    #[endpoint(unwrapEgld)]
    fn unwrap_egld(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();

        require!(payment_token == wrapped_egld_token_id, "Wrong esdt token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            payment_amount <= self.get_locked_egld_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .esdt_local_burn(&wrapped_egld_token_id, 0, &payment_amount);

        // 1 wrapped eGLD = 1 eGLD, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_egld(&caller, &payment_amount, &[]);
    }

    #[view(getLockedEgldBalance)]
    fn get_locked_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::egld(), 0)
    }

    // storage

    #[view(getWrappedEgldTokenIdentifier)]
    #[storage_mapper("wrappedEgldTokenId")]
    fn wrapped_egld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

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

    #[event("wrap-egld")]
    fn wrap_egld_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);

    #[event("unwrap-egld")]
    fn unwrap_egld_event(&self, #[indexed] user: &ManagedAddress, amount: &BigUint);
}
