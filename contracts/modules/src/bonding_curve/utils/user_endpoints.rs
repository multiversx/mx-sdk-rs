multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc::contract_base::ManagedSerializer;

use crate::bonding_curve::{
    curves::curve_function::CurveFunction,
    utils::{events, storage, structs::BondingCurve},
};

#[multiversx_sc::module]
pub trait UserEndpointsModule: storage::StorageModule + events::EventsModule {
    fn sell_token<T>(&self)
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let (offered_token, nonce, sell_amount) = self.call_value().single_esdt().into_tuple();
        let _ = self.check_owned_return_payment_token::<T>(&offered_token, &sell_amount);

        let (calculated_price, payment_token) =
            self.bonding_curve(&offered_token).update(|buffer| {
                let serializer = ManagedSerializer::new();

                let mut bonding_curve: BondingCurve<Self::Api, T> =
                    serializer.top_decode_from_managed_buffer(buffer);

                require!(
                    bonding_curve.sell_availability,
                    "Selling is not available on this token"
                );
                let price = self.compute_sell_price::<T>(&offered_token, &sell_amount);
                bonding_curve.payment.amount -= &price;
                bonding_curve.arguments.balance += &sell_amount;
                let payment_token = bonding_curve.payment_token();
                *buffer = serializer.top_encode_to_managed_buffer(&bonding_curve);
                (price, payment_token)
            });

        let caller = self.blockchain().get_caller();

        self.nonce_amount(&offered_token, nonce)
            .update(|val| *val += sell_amount);

        self.send()
            .direct(&caller, &payment_token, 0u64, &calculated_price);
        self.token_details(&offered_token)
            .update(|details| details.add_nonce(nonce));

        self.sell_token_event(&caller, &calculated_price);
    }

    fn buy_token<T>(
        &self,
        requested_amount: BigUint,
        requested_token: TokenIdentifier,
        requested_nonce: OptionalValue<u64>,
    ) where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let (offered_token, payment) = self.call_value().egld_or_single_fungible_esdt();
        let payment_token =
            self.check_owned_return_payment_token::<T>(&requested_token, &requested_amount);
        self.check_given_token(&payment_token, &offered_token);

        let calculated_price = self.bonding_curve(&requested_token).update(|buffer| {
            let serializer = ManagedSerializer::new();

            let mut bonding_curve: BondingCurve<Self::Api, T> =
                serializer.top_decode_from_managed_buffer(buffer);

            let price = self.compute_buy_price::<T>(&requested_token, &requested_amount);
            require!(
                price <= payment,
                "The payment provided is not enough for the transaction"
            );
            bonding_curve.payment.amount += &price;
            bonding_curve.arguments.balance -= &requested_amount;
            *buffer = serializer.top_encode_to_managed_buffer(&bonding_curve);

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
        let mut tokens_to_send = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();
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
            tokens_to_send.push(EsdtTokenPayment::new(token.clone(), nonce, amount_to_send));
            if total_amount == BigUint::zero() {
                break;
            }
        }

        self.send().direct_multi(caller, &tokens_to_send);

        self.token_details(&token)
            .update(|token_ownership| token_ownership.token_nonces = nonces);
    }

    fn get_buy_price<T>(&self, amount: BigUint, identifier: TokenIdentifier) -> BigUint
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        self.check_token_exists(&identifier);
        self.compute_buy_price::<T>(&identifier, &amount)
    }

    fn get_sell_price<T>(&self, amount: BigUint, identifier: TokenIdentifier) -> BigUint
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        self.check_token_exists(&identifier);
        self.compute_sell_price::<T>(&identifier, &amount)
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

    fn check_owned_return_payment_token<T>(
        &self,
        issued_token: &TokenIdentifier,
        amount: &BigUint,
    ) -> EgldOrEsdtTokenIdentifier
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        self.check_token_exists(issued_token);

        let serializer = ManagedSerializer::new();
        let bonding_curve: BondingCurve<Self::Api, T> =
            serializer.top_decode_from_managed_buffer(&self.bonding_curve(issued_token).get());

        require!(
            bonding_curve.curve != T::default(),
            "The token price was not set yet!"
        );
        require!(amount > &BigUint::zero(), "Must pay more than 0 tokens!");
        bonding_curve.payment_token()
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

    fn compute_buy_price<T>(&self, identifier: &TokenIdentifier, amount: &BigUint) -> BigUint
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let serializer = ManagedSerializer::new();
        let bonding_curve: BondingCurve<Self::Api, T> =
            serializer.top_decode_from_managed_buffer(&self.bonding_curve(identifier).get());

        let arguments = &bonding_curve.arguments;
        let function_selector = &bonding_curve.curve;

        let token_start = &arguments.first_token_available();
        function_selector.calculate_price(token_start, amount, arguments)
    }

    fn compute_sell_price<T>(&self, identifier: &TokenIdentifier, amount: &BigUint) -> BigUint
    where
        T: CurveFunction<Self::Api>
            + TopEncode
            + TopDecode
            + NestedEncode
            + NestedDecode
            + TypeAbi
            + PartialEq
            + Default,
    {
        let serializer = ManagedSerializer::new();
        let bonding_curve: BondingCurve<Self::Api, T> =
            serializer.top_decode_from_managed_buffer(&self.bonding_curve(identifier).get());

        let arguments = &bonding_curve.arguments;
        let function_selector = &bonding_curve.curve;

        let token_start = arguments.first_token_available() - amount;
        function_selector.calculate_price(&token_start, amount, arguments)
    }
}
