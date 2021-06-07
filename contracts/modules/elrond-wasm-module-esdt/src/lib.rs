#![no_std]
#![allow(clippy::string_lit_as_bytes)]

elrond_wasm::imports!();

/// Standard module for managing a single ESDT.
#[elrond_wasm_derive::module]
pub trait EsdtModule {
	#[storage_mapper("token_id")]
	fn token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

	#[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
		token_name: BoxedBytes,
		token_ticker: BoxedBytes,
        #[payment] issue_cost: Self::BigUint,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        only_owner!(self, "only owner can issue new tokens");
        require!(
            self.stablecoin_token_id().is_empty(),
            "Stablecoin already issued"
        );

        let token_display_name = BoxedBytes::from(STABLE_COIN_NAME);
        let token_ticker = BoxedBytes::from(STABLE_COIN_TICKER);
        let initial_supply = Self::BigUint::from(1u32);

        Ok(ESDTSystemSmartContractProxy::new_proxy_obj(self.send())
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    can_burn: true,
                    can_mint: true,
                    num_decimals: 0,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(self.callbacks().stablecoin_issue_callback()))
    }
}
