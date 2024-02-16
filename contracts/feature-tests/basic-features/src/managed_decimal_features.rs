multiversx_sc::imports!();

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
}
