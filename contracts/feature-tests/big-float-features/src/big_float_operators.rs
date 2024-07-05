multiversx_sc::imports!();

/// Checks that BigFloat operators work as expected.
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait BigFloatOperators {
    #[endpoint]
    fn add_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        a + b
    }
    #[endpoint]
    fn add_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        a + b
    }
    #[endpoint]
    fn sub_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        a - b
    }
    #[endpoint]
    fn sub_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        a - b
    }
    #[endpoint]
    fn mul_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        a * b
    }
    #[endpoint]
    fn mul_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        a * b
    }
    #[endpoint]
    fn div_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        a / b
    }
    #[endpoint]
    fn div_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        a / b
    }

    #[endpoint]
    fn add_assign_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn add_assign_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn sub_assign_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn sub_assign_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn div_assign_big_float(&self, a: BigFloat, b: BigFloat) -> BigFloat {
        let mut r = a.clone();
        r /= b;
        r
    }
    #[endpoint]
    fn div_assign_big_float_ref(&self, a: &BigFloat, b: &BigFloat) -> BigFloat {
        let mut r = a.clone();
        r /= b;
        r
    }
}
