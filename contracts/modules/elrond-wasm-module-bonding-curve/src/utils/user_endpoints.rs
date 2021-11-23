elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    curves::curve_function::CurveFunction,
    function_selector::FunctionSelector,
    utils::{events, storage, structs::CurveArguments},
};

#[elrond_wasm::module]
pub trait UserEndpointsModule: storage::StorageModule + events::EventsModule {
    #[payable("*")]
    #[endpoint(sellToken)]
    fn sell_token(
        &self,
        #[payment_amount] sell_amount: BigUint,
        #[payment_nonce] nonce: u64,
        #[payment_token] offered_token: TokenIdentifier,
    ) -> SCResult<()> {
        let _ = self.check_owned_return_payment_token(&offered_token, &sell_amount)?;

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
            bonding_curve.payment_amount -= price.clone()?;
            bonding_curve.arguments.balance += sell_amount.clone();
            price
        })?;

        let caller = self.blockchain().get_caller();

        self.nonce_amount(&offered_token, nonce)
            .update(|val| *val += sell_amount);

        self.send().direct(
            &caller,
            &self.bonding_curve(&offered_token).get().payment_token,
            0u64,
            &calculated_price,
            b"selling",
        );
        self.token_details(&offered_token)
            .update(|details| details.add_nonce(nonce));

        self.sell_token_event(&caller, &calculated_price);

        Ok(())
    }

    #[payable("*")]
    #[endpoint(buyToken)]
    fn buy_token(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] offered_token: TokenIdentifier,
        requested_amount: BigUint,
        requested_token: TokenIdentifier,
        #[var_args] requested_nonce: OptionalArg<u64>,
    ) -> SCResult<()> {
        let payment_token =
            self.check_owned_return_payment_token(&requested_token, &requested_amount)?;
        self.check_given_token(&payment_token, &offered_token);

        let calculated_price = self
            .bonding_curve(&requested_token)
            .update(|bonding_curve| {
                let price = self.compute_buy_price(
                    &bonding_curve.curve,
                    requested_amount.clone(),
                    bonding_curve.arguments.clone(),
                );
                let price_clone = price.clone()?;
                require!(
                    price_clone <= payment,
                    "The payment provided is not enough for the transaction"
                );
                bonding_curve.payment_amount += price_clone;
                bonding_curve.arguments.balance -= &requested_amount;

                price
            })?;

        let caller = self.blockchain().get_caller();

        match requested_nonce {
            OptionalArg::Some(nonce) => {
                self.send().direct(
                    &caller,
                    &requested_token,
                    nonce,
                    &requested_amount,
                    b"buying",
                );
                if self.nonce_amount(&requested_token, nonce).get() - requested_amount.clone() > 0 {
                    self.nonce_amount(&requested_token, nonce)
                        .update(|val| *val -= requested_amount.clone());
                } else {
                    self.nonce_amount(&requested_token, nonce).clear();
                    self.token_details(&requested_token)
                        .update(|details| details.remove_nonce(nonce))?;
                }
            },
            OptionalArg::None => {
                self.send_bought_tokens(&caller, requested_token, requested_amount)?;
            },
        };

        self.send().direct(
            &caller,
            &offered_token,
            0u64,
            &(&payment - &calculated_price),
            b"rest",
        );

        self.buy_token_event(&caller, &calculated_price);
        Ok(())
    }

    fn send_bought_tokens(
        &self,
        caller: &ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) -> SCResult<()> {
        let mut nonces = self.token_details(&token).get().token_nonces;
        let mut total_amount = amount;
        loop {
            let nonce = *nonces.first().ok_or("Requested nonce does not exist")?;
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
                .direct(caller, &token, nonce, &amount_to_send, b"buying");
            if total_amount == 0 {
                break;
            }
        }
        self.token_details(&token)
            .update(|token_ownership| token_ownership.token_nonces = nonces);
        Ok(())
    }

    #[view]
    fn get_buy_price(&self, amount: BigUint, identifier: TokenIdentifier) -> SCResult<BigUint> {
        self.check_token_exists(&identifier)?;

        let bonding_curve = self.bonding_curve(&identifier).get();
        self.compute_buy_price(&bonding_curve.curve, amount, bonding_curve.arguments)
    }

    #[view]
    fn get_sell_price(&self, amount: BigUint, identifier: TokenIdentifier) -> SCResult<BigUint> {
        self.check_token_exists(&identifier)?;

        let bonding_curve = self.bonding_curve(&identifier).get();
        self.compute_sell_price(&bonding_curve.curve, amount, bonding_curve.arguments)
    }

    fn check_token_exists(&self, issued_token: &TokenIdentifier) -> SCResult<()> {
        require!(
            !self.bonding_curve(issued_token).is_empty(),
            "Token is not issued yet!"
        );

        Ok(())
    }

    #[view(getTokenAvailability)]
    fn get_token_availability(
        &self,
        identifier: TokenIdentifier,
    ) -> MultiResultVec<MultiResult2<u64, BigUint>> {
        let token_nonces = self.token_details(&identifier).get().token_nonces;
        let mut availability = Vec::new();

        for current_check_nonce in token_nonces {
            availability.push(MultiArg2((
                current_check_nonce,
                self.nonce_amount(&identifier, current_check_nonce).get(),
            )));
        }
        availability.into()
    }

    fn check_owned_return_payment_token(
        &self,
        issued_token: &TokenIdentifier,
        amount: &BigUint,
    ) -> SCResult<TokenIdentifier> {
        self.check_token_exists(issued_token)?;

        let bonding_curve = self.bonding_curve(issued_token).get();

        require!(
            bonding_curve.curve.is_none(),
            "The token price was not set yet!"
        );
        require!(amount > &0, "Must pay more than 0 tokens!");
        Ok(bonding_curve.payment_token)
    }

    fn check_given_token(&self, accepted_token: &TokenIdentifier, given_token: &TokenIdentifier) {
        if given_token != accepted_token {
            let mut err = self.error().new_error();
            err.append_bytes(&b"Only"[..]);
            err.append_bytes(accepted_token.to_esdt_identifier().as_slice());
            err.append_bytes(&b" tokens accepted"[..]);
            err.exit_now()
        }
    }

    fn compute_buy_price(
        &self,
        function_selector: &FunctionSelector<Self::Api>,
        amount: BigUint,
        arguments: CurveArguments<Self::Api>,
    ) -> SCResult<BigUint> {
        let token_start = arguments.first_token_available();
        function_selector.calculate_price(&token_start, &amount, &arguments)
    }

    fn compute_sell_price(
        &self,
        function_selector: &FunctionSelector<Self::Api>,
        amount: BigUint,
        arguments: CurveArguments<Self::Api>,
    ) -> SCResult<BigUint> {
        let token_start = &arguments.first_token_available() - &amount;
        function_selector.calculate_price(&token_start, &amount, &arguments)
    }
}
