#![no_std]

multiversx_sc::imports!();

const EGLD_DECIMALS: usize = 18;

#[multiversx_sc::contract]
pub trait Child {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint(issueWrappedEgld)]
    fn issue_wrapped_egld(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        initial_supply: BigUint,
    ) {
        let issue_cost = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost.clone_value(),
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: EGLD_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: false,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback())
            .call_and_exit()
    }

    // callbacks

    #[callback]
    fn esdt_issue_callback(&self, #[call_result] _result: IgnoreValue) {
        let (token_identifier, _amount) = self.call_value().single_fungible_esdt();
        self.wrapped_egld_token_identifier().set(&token_identifier);
    }

    // storage

    #[view(getWrappedEgldTokenIdentifier)]
    #[storage_mapper("wrappedEgldTokenIdentifier")]
    fn wrapped_egld_token_identifier(&self) -> SingleValueMapper<TokenIdentifier>;
}
