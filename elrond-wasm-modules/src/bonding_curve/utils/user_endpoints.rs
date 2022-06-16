elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::bonding_curve::{
    curves::curve_function::CurveFunction,
    function_selector::FunctionSelector,
    utils::{events, storage, structs::CurveArguments},
};

#[elrond_wasm::module]
pub trait UserEndpointsModule: storage::StorageModule + events::EventsModule {
    #[payable("*")]
    #[endpoint(sellToken)]
    fn sell_token(&self) {
        let (offered_token, nonce, sell_amount) = self.call_value().single_esdt().into_tuple();
        let _ = self.check_owned_return_payment_token(&offered_token, &sell_amount);

        let calculated_price = self.bonding_curve(&offered_token).update(|bonding_curve| {
            require!(
                bonding_curve.sell_availability,
                "Selling is not available on this token"
            );
            let price = self.compute_sell_price(
                &bonding_curve.curve,
                sell_amount.clone(),
                bonding_curve.arguments.clone(),
            );
            bonding_curve.payment_amount -= price.clone();
            bonding_curve.arguments.balance += sell_amount.clone();
            price
        });

        let caller = self.blockchain().get_caller();

        self.nonce_amount(&offered_token, nonce)
            .update(|val| *val += sell_amount);

        self.send().direct(
            &caller,
            &self.bonding_curve(&offered_token).get().payment_token,
            0u64,
            &calculated_price,
        );
        self.token_details(&offered_token)
            .update(|details| details.add_nonce(nonce));

        self.sell_token_event(&caller, &calculated_price);
    }

    #[payable("*")]
    #[endpoint(buyToken)]
    fn buy_token(
        &self,
        requested_amount: BigUint,
        requested_token: TokenIdentifier,
        requested_nonce: OptionalValue<u64>,
    ) {
        let (offered_token, payment) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token =
            self.check_owned_return_payment_token(&requested_token, &requested_amount);
        self.check_given_token(&payment_token, &offered_token);

        let calculated_price = self
            .bonding_curve(&requested_token)
            .update(|bonding_curve| {
                let price = self.compute_buy_price(
                    &bonding_curve.curve,
                    requested_amount.clone(),
                    bonding_curve.arguments.clone(),
                );
                require!(
                    price <= payment,
                    "The payment provided is not enough for the transaction"
                );
                bonding_curve.payment_amount += &price;
                bonding_curve.arguments.balance -= &requested_amount;

                price
            });

        let caller = self.blockchain().get_caller();

        match requested_nonce {
            OptionalValue::Some(nonce) => {
                self.send()
                    .direct_esdt(&caller, &requested_token, nonce, &requested_amount);
                if self.nonce_amount(&requested_token, nonce).get() - requested_amount.clone() > 0 {
                    self.nonce_amount(&requested_token, nonce)
                        .update(|val| *val -= requested_amount.clone());
                } else {
                    self.nonce_amount(&requested_token, nonce).clear();
                    self.token_details(&requested_token)
                        .update(|details| details.remove_nonce(nonce));
                }
            },
            OptionalValue::None => {
                self.send_next_available_tokens(&caller, requested_token, requested_amount);
            },
        };

        self.send().direct(
            &caller,
            &offered_token,
            0u64,
            &(&payment - &calculated_price),
        );

        self.buy_token_event(&caller, &calculated_price);
    }

    fn send_next_available_tokens(
        &self,
        caller: &ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) {
        let mut nonces = self.token_details(&token).get().token_nonces;
        let mut total_amount = amount;
        loop {
            require!(!nonces.is_empty(), "Insufficient balance");
            let nonce = nonces.get(0);
            let available_amount = self.nonce_amount(&token, nonce).get();

            let amount_to_send: BigUint;
            if available_amount <= total_amount {
                amount_to_send = available_amount.clone();
                total_amount -= amount_to_send.clone();
                self.nonce_amount(&token, nonce).clear();
                nonces.remove(0);
            } else {
                self.nonce_amount(&token, nonce)
                    .update(|val| *val -= total_amount.clone());
                amount_to_send = total_amount.clone();
                total_amount = BigUint::zero();
            }
            self.send()
                .direct_esdt(caller, &token, nonce, &amount_to_send);
            if total_amount == BigUint::zero() {
                break;
            }
        }

        self.token_details(&token)
            .update(|token_ownership| token_ownership.token_nonces = nonces);
    }

    #[view]
    fn get_buy_price(&self, amount: BigUint, identifier: TokenIdentifier) -> BigUint {
        self.check_token_exists(&identifier);

        let bonding_curve = self.bonding_curve(&identifier).get();
        self.compute_buy_price(&bonding_curve.curve, amount, bonding_curve.arguments)
    }

    #[view]
    fn get_sell_price(&self, amount: BigUint, identifier: TokenIdentifier) -> BigUint {
        self.check_token_exists(&identifier);

        let bonding_curve = self.bonding_curve(&identifier).get();
        self.compute_sell_price(&bonding_curve.curve, amount, bonding_curve.arguments)
    }

    fn check_token_exists(&self, issued_token: &TokenIdentifier) {
        require!(
            !self.bonding_curve(issued_token).is_empty(),
            "Token is not issued yet!"
        );
    }

    #[view(getTokenAvailability)]
    fn get_token_availability(
        &self,
        identifier: TokenIdentifier,
    ) -> MultiValueEncoded<MultiValue2<u64, BigUint>> {
        let token_nonces = self.token_details(&identifier).get().token_nonces;
        let mut availability = MultiValueEncoded::new();

        for current_check_nonce in &token_nonces {
            availability.push(MultiValue2((
                current_check_nonce,
                self.nonce_amount(&identifier, current_check_nonce).get(),
            )));
        }
        availability
    }

    fn check_owned_return_payment_token(
        &self,
        issued_token: &TokenIdentifier,
        amount: &BigUint,
    ) -> EgldOrEsdtTokenIdentifier {
        self.check_token_exists(issued_token);

        let bonding_curve = self.bonding_curve(issued_token).get();

        require!(
            bonding_curve.curve.is_none(),
            "The token price was not set yet!"
        );
        require!(amount > &0, "Must pay more than 0 tokens!");
        bonding_curve.payment_token
    }

    fn check_given_token(
        &self,
        accepted_token: &EgldOrEsdtTokenIdentifier,
        given_token: &EgldOrEsdtTokenIdentifier,
    ) {
        require!(
            given_token == accepted_token,
            "Only {} tokens accepted",
            accepted_token
        );
    }

    fn compute_buy_price(
        &self,
        function_selector: &FunctionSelector<Self::Api>,
        amount: BigUint,
        arguments: CurveArguments<Self::Api>,
    ) -> BigUint {
        let token_start = arguments.first_token_available();
        function_selector.calculate_price(&token_start, &amount, &arguments)
    }

    fn compute_sell_price(
        &self,
        function_selector: &FunctionSelector<Self::Api>,
        amount: BigUint,
        arguments: CurveArguments<Self::Api>,
    ) -> BigUint {
        let token_start = &arguments.first_token_available() - &amount;
        function_selector.calculate_price(&token_start, &amount, &arguments)
    }
}
