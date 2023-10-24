multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BigIntMethods {
    #[endpoint]
    fn sqrt_big_uint(&self, a: BaseBigUint) -> BaseBigUint {
        a.sqrt()
    }

    #[endpoint]
    fn sqrt_big_uint_ref(&self, a: &BaseBigUint) -> BaseBigUint {
        a.sqrt()
    }

    #[endpoint]
    fn log2_big_uint(&self, a: BaseBigUint) -> u32 {
        a.log2()
    }

    #[endpoint]
    fn log2_big_uint_ref(&self, a: &BaseBigUint) -> u32 {
        a.log2()
    }

    #[endpoint]
    fn pow_big_int(&self, a: BigInt, b: u32) -> BigInt {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_int_ref(&self, a: &BigInt, b: u32) -> BigInt {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_uint(&self, a: BaseBigUint, b: u32) -> BaseBigUint {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_uint_ref(&self, a: &BaseBigUint, b: u32) -> BaseBigUint {
        a.pow(b)
    }

    #[endpoint]
    fn big_uint_to_u64(&self, bu: &BaseBigUint) -> OptionalValue<u64> {
        bu.to_u64().into()
    }

    #[endpoint]
    fn biguint_overwrite_u64(&self, bu: BaseBigUint, small: u64) -> BaseBigUint {
        bu.overwrite_u64(small);
        bu
    }

    #[endpoint]
    fn big_uint_zero(&self) -> BaseBigUint {
        BaseBigUint::zero()
    }

    #[endpoint]
    fn big_uint_from_u64_1(&self, small: u64) -> BaseBigUint {
        BaseBigUint::from(small)
    }

    #[endpoint]
    fn big_uint_from_u64_2(&self, small: u64) -> BaseBigUint {
        small.into()
    }

    #[endpoint]
    fn biguint_from_u128(&self) -> BaseBigUint {
        BaseBigUint::from(u128::MAX)
    }

    #[endpoint]
    fn big_uint_from_managed_buffer(&self, mb: ManagedBuffer) -> BaseBigUint {
        BaseBigUint::from(mb)
    }

    #[endpoint]
    fn big_uint_from_managed_buffer_ref(&self, mb: &ManagedBuffer) -> BaseBigUint {
        BaseBigUint::from(mb)
    }

    #[endpoint]
    fn big_int_zero(&self) -> BigInt {
        BigInt::zero()
    }

    #[endpoint]
    fn big_int_from_i64_1(&self, small: i64) -> BigInt {
        BigInt::from(small)
    }

    #[endpoint]
    fn big_int_from_i64_2(&self, small: i64) -> BigInt {
        small.into()
    }

    #[endpoint]
    fn big_uint_eq_u64(&self, bi: BaseBigUint, small: u64) -> bool {
        bi == small
    }

    #[endpoint]
    fn big_int_to_i64(&self, bi: &BigInt) -> OptionalValue<i64> {
        bi.to_i64().into()
    }

    #[endpoint]
    fn bigint_overwrite_i64(&self, bi: BigInt, small: i64) -> BigInt {
        bi.overwrite_i64(small);
        bi
    }

    #[endpoint]
    fn big_int_to_parts(&self, bi: BigInt) -> MultiValue2<Sign, BaseBigUint> {
        bi.to_parts().into()
    }

    #[endpoint]
    fn big_int_from_biguint(&self, sign: Sign, unsigned: BaseBigUint) -> BigInt {
        BigInt::from_biguint(sign, unsigned)
    }
}
