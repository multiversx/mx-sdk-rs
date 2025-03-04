use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ManagedDecimalFeatures {
    #[endpoint]
    fn managed_decimal_addition(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
    ) -> ManagedDecimal<Self::Api, ConstDecimals<U2>> {
        first + second
    }

    #[endpoint]
    fn managed_decimal_subtraction(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
    ) -> ManagedDecimal<Self::Api, ConstDecimals<U2>> {
        first - second
    }

    #[endpoint]
    fn managed_decimal_eq(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<U2>>,
    ) -> bool {
        first.eq(&second)
    }

    #[endpoint]
    fn managed_decimal_trunc(&self) -> BigUint {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(31332u64), 2usize);
        dec.trunc()
    }

    #[endpoint]
    fn managed_decimal_into_raw_units(&self) -> BigUint {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(12345u64), 2usize);
        dec.into_raw_units().clone()
    }

    #[endpoint]
    fn managed_decimal_ln(
        &self,
        x: ManagedDecimal<Self::Api, LnDecimals>,
    ) -> ManagedDecimalSigned<Self::Api, LnDecimals> {
        x.ln().unwrap_or_else(|| sc_panic!("cannot be zero"))
    }

    #[endpoint]
    fn managed_decimal_log2(
        &self,
        x: ManagedDecimal<Self::Api, LnDecimals>,
    ) -> ManagedDecimalSigned<Self::Api, LnDecimals> {
        x.log2().unwrap_or_else(|| sc_panic!("cannot be zero"))
    }

    #[endpoint]
    fn managed_decimal_addition_var(
        &self,
        first: ManagedDecimal<Self::Api, NumDecimals>,
        second: ManagedDecimal<Self::Api, NumDecimals>,
    ) -> ManagedDecimal<Self::Api, NumDecimals> {
        first + second
    }

    #[endpoint]
    fn managed_decimal_subtraction_var(
        &self,
        first: ManagedDecimal<Self::Api, NumDecimals>,
        second: ManagedDecimal<Self::Api, NumDecimals>,
    ) -> ManagedDecimal<Self::Api, NumDecimals> {
        first - second
    }

    #[endpoint]
    fn managed_decimal_eq_var(
        &self,
        first: ManagedDecimal<Self::Api, NumDecimals>,
        second: ManagedDecimal<Self::Api, NumDecimals>,
    ) -> bool {
        first.eq(&second)
    }

    #[endpoint]
    fn managed_decimal_ln_var(
        &self,
        x: ManagedDecimal<Self::Api, NumDecimals>,
    ) -> ManagedDecimalSigned<Self::Api, LnDecimals> {
        x.ln().unwrap_or_else(|| sc_panic!("cannot be zero"))
    }

    #[endpoint]
    fn managed_decimal_log2_var(
        &self,
        x: ManagedDecimal<Self::Api, NumDecimals>,
    ) -> ManagedDecimalSigned<Self::Api, LnDecimals> {
        x.log2().unwrap_or_else(|| sc_panic!("cannot be zero"))
    }
}
