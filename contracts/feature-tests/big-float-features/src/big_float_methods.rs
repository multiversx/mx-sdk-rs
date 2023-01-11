multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BigFloatMethods {
    #[endpoint]
    fn new_from_parts_big_float(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> BigFloat {
        BigFloat::from_parts(integral_part_value, fractional_part_value, exponent_value)
    }

    #[endpoint]
    fn new_from_frac_big_float(&self, numerator_value: i64, denominator_value: i64) -> BigFloat {
        BigFloat::from_frac(numerator_value, denominator_value)
    }

    #[endpoint]
    fn new_from_sci_big_float(&self, significand_value: i64, exponent_value: i32) -> BigFloat {
        BigFloat::from_sci(significand_value, exponent_value)
    }

    #[endpoint]
    fn big_float_from_big_uint_1(&self, bi: BigUint) -> BigFloat {
        BigFloat::from_big_uint(&bi)
    }

    #[endpoint]
    fn big_float_from_big_uint_2(&self, bi: BigUint) -> BigFloat {
        BigFloat::from(bi)
    }

    #[endpoint]
    fn big_float_from_big_int_1(&self, bi: BigInt) -> BigFloat {
        BigFloat::from_big_int(&bi)
    }

    #[endpoint]
    fn big_float_from_big_int_2(&self, bi: BigInt) -> BigFloat {
        BigFloat::from(bi)
    }

    #[endpoint]
    fn big_float_from_i64(&self, small_value: i64) -> BigFloat {
        BigFloat::from(small_value)
    }

    #[endpoint]
    fn big_float_from_i32(&self, small_value: i16) -> BigFloat {
        BigFloat::from(small_value)
    }

    #[endpoint]
    fn big_float_from_i16(&self, small_value: i32) -> BigFloat {
        BigFloat::from(small_value)
    }

    #[endpoint]
    fn big_float_from_i8(&self, small_value: i8) -> BigFloat {
        BigFloat::from(small_value)
    }

    #[endpoint]
    fn big_float_from_isize(&self, small_value: isize) -> BigFloat {
        BigFloat::from(small_value)
    }

    #[endpoint]
    fn big_float_from_man_buf(&self, man_buf: ManagedBuffer) -> BigFloat {
        BigFloat::from(man_buf)
    }

    #[endpoint]
    fn big_float_from_man_buf_ref(&self, man_buf: &ManagedBuffer) -> BigFloat {
        BigFloat::from(man_buf)
    }

    #[endpoint]
    fn sqrt_big_float(&self, a: BigFloat) -> BigFloat {
        a.sqrt()
    }

    #[endpoint]
    fn sqrt_big_float_ref(&self, a: &BigFloat) -> BigFloat {
        a.sqrt()
    }

    #[endpoint]
    fn pow_big_float(&self, a: BigFloat, b: i32) -> BigFloat {
        a.pow(b)
    }

    #[endpoint]
    fn pow_big_float_ref(&self, a: &BigFloat, b: i32) -> BigFloat {
        a.pow(b)
    }

    #[endpoint]
    fn big_float_zero(&self) -> BigFloat {
        BigFloat::zero()
    }

    #[endpoint]
    fn big_float_neg(&self, a: BigFloat) -> BigFloat {
        a.neg()
    }
}
