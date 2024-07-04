multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BigFloatWrappedLn {
    #[endpoint]
    fn ln_big_float_ref(&self, a: &BigFloat) -> BigFloat {
        a.ln()
            .unwrap_or_else(|| sc_panic!("log argument must pe strictly positive"))
    }

    #[endpoint]
    fn ln_big_float_ref_wrapped(
        &self,
        a: BigInt,
        precision: usize,
    ) -> ManagedDecimalSigned<Self::Api, usize> {
        let number = self.ln_big_float_ref(&BigFloat::from(a));
        number.to_managed_decimal_signed(precision)
    }
}
