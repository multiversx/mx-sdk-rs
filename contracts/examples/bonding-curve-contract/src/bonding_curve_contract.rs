#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use function_selector::FunctionSelector;
use multiversx_sc_modules::{
    bonding_curve,
    bonding_curve::utils::{events, owner_endpoints, storage, user_endpoints},
};
pub mod function_selector;

#[multiversx_sc::contract]
pub trait Contract:
    bonding_curve::BondingCurveModule
    + storage::StorageModule
    + events::EventsModule
    + user_endpoints::UserEndpointsModule
    + owner_endpoints::OwnerEndpointsModule
{
    #[init]
    fn init(&self) {}

    #[payable("*")]
    #[endpoint(sellToken)]
    fn sell_token_endpoint(&self) {
        self.sell_token::<FunctionSelector<CurrentApi>>();
    }

    #[payable("*")]
    #[endpoint(buyToken)]
    fn buy_token_endpoint(
        &self,
        requested_amount: BaseBigUint,
        requested_token: TokenIdentifier,
        requested_nonce: OptionalValue<u64>,
    ) {
        self.buy_token::<FunctionSelector<CurrentApi>>(
            requested_amount,
            requested_token,
            requested_nonce,
        );
    }

    #[endpoint(deposit)]
    #[payable("*")]
    fn deposit_endpoint(&self, payment_token: OptionalValue<TokenIdentifier>) {
        self.deposit::<FunctionSelector<CurrentApi>>(payment_token)
    }

    #[endpoint(setBondingCurve)]
    fn set_bonding_curve_endpoint(
        &self,
        identifier: TokenIdentifier,
        function: FunctionSelector<CurrentApi>,
        sell_availability: bool,
    ) {
        self.set_bonding_curve::<FunctionSelector<CurrentApi>>(
            identifier,
            function,
            sell_availability,
        );
    }
    #[endpoint(claim)]
    fn claim_endpoint(&self) {
        self.claim::<FunctionSelector<CurrentApi>>();
    }

    #[view]
    fn view_buy_price(&self, amount: BaseBigUint, identifier: TokenIdentifier) -> BaseBigUint {
        self.get_buy_price::<FunctionSelector<CurrentApi>>(amount, identifier)
    }

    #[view]
    fn view_sell_price(&self, amount: BaseBigUint, identifier: TokenIdentifier) -> BaseBigUint {
        self.get_sell_price::<FunctionSelector<CurrentApi>>(amount, identifier)
    }
}
