elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait BigFloatWrappedEndpoints {
    #[endpoint]
    fn new_from_parts_big_float_multiplied(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number =
            BigFloat::from_parts(integral_part_value, fractional_part_value, exponent_value);
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn new_from_frac_big_float_multiplied(
        &self,
        numerator_value: i64,
        denominator_value: i64,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = BigFloat::from_frac(numerator_value, denominator_value);
        let mut result = number.to_fixed_point(&BigFloat::from(fixed_point_denominator));
        result = result * 10i64.into();
        result
    }
}
