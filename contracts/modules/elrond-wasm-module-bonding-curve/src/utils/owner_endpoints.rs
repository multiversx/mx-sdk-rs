elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
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
        #[var_args] roles: ManagedVarArgs<EsdtLocalRole>,
    ) -> AsyncCall {
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
    }

    #[endpoint(unsetLocalRoles)]
    fn unset_local_roles(
        &self,
        address: ManagedAddress,
        token_identifier: TokenIdentifier,
        #[var_args] roles: ManagedVarArgs<EsdtLocalRole>,
    ) -> AsyncCall {
        self.send()
            .esdt_system_sc_proxy()
            .unset_special_roles(&address, &token_identifier, roles.into_iter())
            .async_call()
            .with_callback(OwnerEndpointsModule::callbacks(self).change_roles_callback())
    }

    #[callback]
    fn change_roles_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) -> SCResult<(), ManagedSCError> {
        match result {
            ManagedAsyncCallResult::Ok(()) => Ok(()),
            ManagedAsyncCallResult::Err(message) => Err(message.err_msg.into()),
        }
    }

    #[endpoint(setBondingCurve)]
    fn set_bonding_curve(
        &self,
        identifier: TokenIdentifier,
        function: FunctionSelector<Self::Api>,
        sell_availability: bool,
    ) -> SCResult<()> {
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
        Ok(())
    }

    #[endpoint(deposit)]
    #[payable("*")]
    fn deposit(
        &self,
        #[payment] amount: BigUint,
        #[payment_token] identifier: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[var_args] payment_token: OptionalArg<TokenIdentifier>,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let mut set_payment = TokenIdentifier::egld();
        if self.bonding_curve(&identifier).is_empty() {
            set_payment = payment_token
                .into_option()
                .ok_or("Expected provided accepted_payment for the token")?;
        }
        if self.token_details(&identifier).is_empty() {
            self.token_details(&identifier).set(&TokenOwnershipData {
                token_nonces: [nonce].to_vec(),
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
        Ok(())
    }

    #[endpoint(claim)]
    fn claim(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        require!(
            !self.owned_tokens(&caller).is_empty(),
            "You have nothing to claim"
        );
        for token in self.owned_tokens(&caller).iter() {
            let nonces = self.token_details(&token).get().token_nonces;
            for nonce in nonces {
                self.send().direct(
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

        Ok(())
    }

    fn set_curve_storage(
        &self,
        identifier: &TokenIdentifier,
        amount: BigUint,
        payment: TokenIdentifier,
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
