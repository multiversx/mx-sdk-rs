use crate::big_float_operators;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait BigFloatWrappedOperators: big_float_operators::BigFloatOperators {
    #[endpoint]
    fn add_big_float_wrapped(&self, a: BigInt, b: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.add_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn add_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.add_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn sub_big_float_wrapped(&self, a: BigInt, b: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.sub_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn sub_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.sub_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn mul_big_float_wrapped(&self, a: BigInt, b: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.mul_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }
    #[endpoint]
    fn mul_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.mul_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn div_big_float_wrapped(&self, a: BigInt, b: BigInt, fixed_point_denominator: i64) -> BigInt {
        let number = self.div_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn div_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.div_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn add_assign_big_float_wrapped(
        &self,
        a: BigInt,
        b: BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.add_assign_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn add_assign_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.add_assign_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn sub_assign_big_float_wrapped(
        &self,
        a: BigInt,
        b: BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.sub_assign_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn sub_assign_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.sub_assign_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn mul_assign_big_float_wrapped(
        &self,
        a: BigInt,
        b: BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.mul_assign_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn mul_assign_big_float_ref_wrapped(
        &self,
        a: &BigInt,
        b: &BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.mul_assign_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn div_assign_big_float_wrapped(
        &self,
        a: BigInt,
        b: BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.div_assign_big_float(BigFloat::from(a), BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }

    #[endpoint]
    fn div_assign_big_float_ref_wrapped(
        &self,
        a: BigInt,
        b: BigInt,
        fixed_point_denominator: i64,
    ) -> BigInt {
        let number = self.div_assign_big_float_ref(&BigFloat::from(a), &BigFloat::from(b));
        number.to_fixed_point(&BigFloat::from(fixed_point_denominator))
    }
}
