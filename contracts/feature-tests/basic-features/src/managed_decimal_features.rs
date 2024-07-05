use multiversx_sc::imports::*;

#[multiversx_sc::module]
pub trait ManagedDecimalFeatures {
    #[endpoint]
    fn managed_decimal_addition(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<2>>,
    ) -> ManagedDecimal<Self::Api, ConstDecimals<2>> {
        first + second
    }

    #[endpoint]
    fn managed_decimal_subtraction(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<2>>,
    ) -> ManagedDecimal<Self::Api, ConstDecimals<2>> {
        first - second
    }

    #[endpoint]
    fn managed_decimal_eq(
        &self,
        first: ManagedDecimal<Self::Api, ConstDecimals<2>>,
        second: ManagedDecimal<Self::Api, ConstDecimals<2>>,
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
    fn managed_decimal_ln(&self) -> ManagedDecimalSigned<Self::Api, ConstDecimals<9>> {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(378298u64), 3usize);
        dec.ln().unwrap()
    }

    #[endpoint]
    fn managed_decimal_ln_high_prec(&self) -> ManagedDecimalSigned<Self::Api, ConstDecimals<9>> {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(378298000000u64), 9usize);
        dec.ln().unwrap()
    }

    #[endpoint]
    fn managed_decimal_log2(&self) -> ManagedDecimalSigned<Self::Api, ConstDecimals<9>> {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(218345u64), 3usize);
        dec.log2().unwrap()
    }

    #[endpoint]
    fn managed_decimal_log2_high_prec(&self) -> ManagedDecimalSigned<Self::Api, ConstDecimals<9>> {
        let dec = ManagedDecimal::from_raw_units(BigUint::from(218345000000u64), 9usize);
        dec.log2().unwrap()
    }
}
