use crate::big_float_methods;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BigFloatWrappedMethods: big_float_methods::BigFloatMethods {
    #[endpoint]
    fn new_from_parts_big_float_wrapped(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.new_from_parts_big_float(
            integral_part_value,
            fractional_part_value,
            exponent_value,
        );
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn new_from_frac_big_float_wrapped(
        &self,
        numerator_value: i64,
        denominator_value: i64,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.new_from_frac_big_float(numerator_value, denominator_value);
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn new_from_sci_big_float_wrapped(
        &self,
        significand_value: i64,
        exponent_value: i32,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.new_from_sci_big_float(significand_value, exponent_value);
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn big_float_from_big_int_1_wrapped(&self, bi: BigInt) -> BigInt {
        let number = self.big_float_from_big_int_1(bi);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_big_int_2_wrapped(&self, bi: BigInt) -> BigInt {
        let number = self.big_float_from_big_int_2(bi);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_big_uint_1_wrapped(&self, bu: BigUint) -> BigInt {
        let number = self.big_float_from_big_uint_1(bu);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_big_uint_2_wrapped(&self, bu: BigUint) -> BigInt {
        let number = self.big_float_from_big_uint_2(bu);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_i64_wrapped(&self, small_value: i64) -> BigInt {
        let number = BigFloat::from(small_value);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_i32_wrapped(&self, small_value: i32) -> BigInt {
        let number = BigFloat::from(small_value);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_i16_wrapped(&self, small_value: i16) -> BigInt {
        let number = BigFloat::from(small_value);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_i8_wrapped(&self, small_value: i8) -> BigInt {
        let number = BigFloat::from(small_value);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_from_isize_wrapped(&self, small_value: isize) -> BigInt {
        let number = BigFloat::from(small_value);
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn sqrt_big_float_wrapped(&self, a: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.sqrt_big_float(BigFloat::from(a));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn sqrt_big_float_ref_wrapped(&self, a: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.sqrt_big_float_ref(&BigFloat::from(a));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn pow_big_float_wrapped(&self, a: BigInt, b: i32, fixed_point_denominator: i64) -> BigInt {
        let number = self.pow_big_float(BigFloat::from(a), b);
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn pow_big_float_ref_wrapped(&self, a: BigInt, b: i32, fixed_point_denominator: i64) -> BigInt {
        let number = self.pow_big_float_ref(&BigFloat::from(a), b);
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn big_float_zero_wrapped(&self) -> BigInt {
        let number = self.big_float_zero();
        number.to_fixed_point(&BigFloat::from(1))
    }

    #[endpoint]
    fn big_float_neg_wrapped(&self, a: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.big_float_neg(BigFloat::from(a));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }
}
