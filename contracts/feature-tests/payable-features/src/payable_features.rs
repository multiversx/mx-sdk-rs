#![no_std]
#![allow(clippy::type_complexity)]

multiversx_sc::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting EGLD and ESDT payments.
#[multiversx_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value(
        &self,
    ) -> MultiValue2<BigUint, ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>>> {
        (
            self.call_value().egld_value().clone_value(),
            self.call_value().all_esdt_transfers().clone_value(),
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedRef<'static, ManagedVec<EsdtTokenPayment<Self::Api>>>,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        payments.clone_value()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_3(&self) -> MultiValue3<EsdtTokenPayment, EsdtTokenPayment, EsdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();
        (payment_a, payment_b, payment_c).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment_amount] payment: BigUint,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let token = self.call_value().egld_or_single_esdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_or_single_esdt();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_or_single_esdt();
        (payment.amount, payment.token_identifier).into()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld_1(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let token = self.call_value().egld_or_single_esdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld_3(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_value().clone_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld_4(&self) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld_value();
        let token = self.call_value().egld_or_single_esdt().token_identifier;
        (payment.clone_value(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: BigUint,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: BigUint,
    ) -> MultiValue2<BigUint, TokenIdentifier> {
        let token = self.call_value().single_esdt().token_identifier;
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().single_esdt();
        (payment.amount, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_esdt().amount;
        let token = self.call_value().single_esdt().token_identifier;
        (payment, token).into()
    }
}
