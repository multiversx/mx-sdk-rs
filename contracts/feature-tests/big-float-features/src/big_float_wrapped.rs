use crate::big_float_methods;

elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait BigFloatWrappedEndpoints: big_float_methods::BigFloatMethods {
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
}
