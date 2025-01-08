#![no_std]
#![allow(clippy::type_complexity)]

use multiversx_sc::imports::*;

pub mod payable_features_proxy;

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting EGLD and ESDT payments.
#[multiversx_sc::contract]
pub trait PayableFeatures {
    #[init]
    fn init(&self) {}

    #[view]
    #[payable("*")]
    fn echo_call_value_legacy(&self) -> MultiValue2<BigUint, ManagedVec<EsdtTokenPayment>> {
        (
            self.call_value().egld_direct_non_strict().clone_value(),
            self.call_value().all_esdt_transfers().clone_value(),
        )
            .into()
    }

    #[view]
    #[payable("*")]
    fn echo_call_value(&self) -> ManagedVec<EgldOrEsdtTokenPayment> {
        self.call_value().all_transfers().clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_multiple(
        &self,
        #[payment_multi] payments: ManagedRef<'static, ManagedVec<EsdtTokenPayment<Self::Api>>>,
    ) -> ManagedVec<EsdtTokenPayment<Self::Api>> {
        payments.clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_all_transfers(&self) -> ManagedVec<EgldOrEsdtTokenPayment> {
        self.call_value().all_transfers().clone()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_esdt_3(
        &self,
    ) -> MultiValue3<EsdtTokenPayment, EsdtTokenPayment, EsdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payment_array_egld_esdt_3(
        &self,
    ) -> MultiValue3<EgldOrEsdtTokenPayment, EgldOrEsdtTokenPayment, EgldOrEsdtTokenPayment> {
        let [payment_a, payment_b, payment_c] = self.call_value().multi_egld_or_esdt();
        (payment_a.clone(), payment_b.clone(), payment_c.clone()).into()
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
        let payment = self.call_value().egld().clone();
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
        let payment = self.call_value().egld().clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld_4(&self) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().egld();
        let token = self.call_value().egld_or_single_esdt().token_identifier;
        (payment.clone(), token).into()
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
        let token = self.call_value().single_esdt().token_identifier.clone();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: EgldOrEsdtTokenIdentifier,
    ) -> MultiValue2<BigUint, EgldOrEsdtTokenIdentifier> {
        let payment = self.call_value().single_esdt();
        (payment.amount.clone(), token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiValue2<BigUint, TokenIdentifier> {
        let payment = self.call_value().single_esdt().amount.clone();
        let token = self.call_value().single_esdt().token_identifier.clone();
        (payment, token).into()
    }
}
