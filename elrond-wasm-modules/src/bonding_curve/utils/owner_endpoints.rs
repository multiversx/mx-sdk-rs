elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bonding_curve::{
    function_selector::FunctionSelector,
    utils::{
        events, storage,
        structs::{BondingCurve, TokenOwnershipData},
    },
};

use super::structs::CurveArguments;

#[elrond_wasm::module]
pub trait OwnerEndpointsModule: storage::StorageModule + events::EventsModule {
    #[endpoint(setLocalRoles)]
    fn set_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        roles: MultiValueEncoded<EsdtLocalRole>,
    ) {
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .call_and_exit()
    }

    #[endpoint(unsetLocalRoles)]
    fn unset_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        roles: MultiValueEncoded<EsdtLocalRole>,
    ) {
        self.send()
            .esdt_system_sc_proxy()
            .unset_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .call_and_exit()
    }

    #[endpoint(setBondingCurve)]
    fn set_bonding_curve(
        &self,
        identifier: TokenIdentifier,
        function: FunctionSelector<Self::Api>,
        sell_availability: bool,
    ) {
        require!(
            !self.token_details(&identifier).is_empty(),
            "Token is not issued yet!"
        );

        let caller = self.blockchain().get_caller();

        let details = self.token_details(&identifier).get();
        require!(
            details.owner == caller,
            "The price function can only be set by the seller."
        );
        self.bonding_curve(&identifier).update(|bonding_curve| {
            bonding_curve.curve = function;
            bonding_curve.sell_availability = sell_availability
        });
    }

    #[endpoint(deposit)]
    #[payable("*")]
    fn deposit(&self, payment_token: OptionalValue<TokenIdentifier>) {
        let (identifier, nonce, amount) = self.call_value().single_esdt().into_tuple();
        let caller = self.blockchain().get_caller();
        let mut set_payment = EgldOrEsdtTokenIdentifier::egld();

        if self.bonding_curve(&identifier).is_empty() {
            match payment_token {
                OptionalValue::Some(token) => set_payment = EgldOrEsdtTokenIdentifier::esdt(token),
                OptionalValue::None => {
                    sc_panic!("Expected provided accepted_payment for the token");
                },
            };
        }
        if self.token_details(&identifier).is_empty() {
            let nonces = ManagedVec::from_single_item(nonce);
            self.token_details(&identifier).set(&TokenOwnershipData {
                token_nonces: nonces,
                owner: caller.clone(),
            });
        } else {
            let mut details = self.token_details(&identifier).get();
            require!(
                details.owner == caller,
                "The token was already deposited by another address"
            );
            if !details.token_nonces.contains(&nonce) {
                details.token_nonces.push(nonce);
                self.token_details(&identifier).set(&details);
            }
        }

        self.set_curve_storage(&identifier, amount.clone(), set_payment);
        self.owned_tokens(&caller).insert(identifier.clone());
        self.nonce_amount(&identifier, nonce)
            .update(|current_amount| *current_amount += amount);
    }

    #[endpoint(claim)]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            !self.owned_tokens(&caller).is_empty(),
            "You have nothing to claim"
        );
        for token in self.owned_tokens(&caller).iter() {
            let nonces = self.token_details(&token).get().token_nonces;
            for nonce in &nonces {
                self.send().direct_esdt(
                    &caller,
                    &token,
                    nonce,
                    &self.nonce_amount(&token, nonce).get(),
                    b"claim",
                );
                self.nonce_amount(&token, nonce).clear();
            }
            self.token_details(&token).clear();
            self.bonding_curve(&token).clear();
        }
        self.owned_tokens(&caller).clear();
    }

    fn set_curve_storage(
        &self,
        identifier: &TokenIdentifier,
        amount: BigUint,
        payment: EgldOrEsdtTokenIdentifier,
    ) {
        let mut curve = FunctionSelector::None;
        let mut arguments;
        let payment_token;
        let payment_amount: BigUint;
        let sell_availability: bool;

        if self.bonding_curve(identifier).is_empty() {
            arguments = CurveArguments {
                available_supply: amount.clone(),
                balance: amount,
            };
            payment_token = payment;
            payment_amount = BigUint::zero();
            sell_availability = false;
        } else {
            let bonding_curve = self.bonding_curve(identifier).get();
            payment_token = bonding_curve.payment_token;
            payment_amount = bonding_curve.payment_amount;
            curve = bonding_curve.curve;
            arguments = bonding_curve.arguments;
            arguments.balance += &amount;
            arguments.available_supply += amount;
            sell_availability = bonding_curve.sell_availability;
        }
        self.bonding_curve(identifier).set(&BondingCurve {
            curve,
            arguments,
            sell_availability,
            payment_token,
            payment_amount,
        });
    }
}
