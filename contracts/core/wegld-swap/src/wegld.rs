#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait EgldEsdtSwap: multiversx_sc_modules::pause::PauseModule {
    #[init]
    fn init(&self, wrapped_egld_token_id: TokenIdentifier) {
        self.wrapped_egld_token_id().set(&wrapped_egld_token_id);
    }

    // endpoints

    #[payable("EGLD")]
    #[endpoint(wrapEgld)]
    fn wrap_egld(&self) -> EsdtTokenPayment<Self::Api> {
        self.require_not_paused();

        let payment_amount = self.call_value().egld_value();
        require!(*payment_amount > 0u32, "Payment must be more than 0");

        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
        self.send()
            .esdt_local_mint(&wrapped_egld_token_id, 0, &payment_amount);

        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &wrapped_egld_token_id, 0, &payment_amount);

        EsdtTokenPayment::new(wrapped_egld_token_id, 0, payment_amount.clone_value())
    }

    #[payable("*")]
    #[endpoint(unwrapEgld)]
    fn unwrap_egld(&self) {
        self.require_not_paused();

        let (payment_token, payment_amount) = self.call_value().single_fungible_esdt();
        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();

        require!(payment_token == wrapped_egld_token_id, "Wrong esdt token");
        require!(payment_amount > 0u32, "Must pay more than 0 tokens!");
        require!(
            payment_amount <= self.get_locked_egld_balance(),
            "Contract does not have enough funds"
        );

        self.send()
            .esdt_local_burn(&wrapped_egld_token_id, 0, &payment_amount);

        // 1 wrapped eGLD = 1 eGLD, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send().direct_egld(&caller, &payment_amount);
    }

    #[view(getLockedEgldBalance)]
    fn get_locked_egld_balance(&self) -> BigUint {
        self.blockchain()
            .get_sc_balance(&EgldOrEsdtTokenIdentifier::egld(), 0)
    }

    #[view(getWrappedEgldTokenId)]
    #[storage_mapper("wrappedEgldTokenId")]
    fn wrapped_egld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
