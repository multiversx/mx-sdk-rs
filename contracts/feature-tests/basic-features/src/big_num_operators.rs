multiversx_sc::imports!();

/// Checks that BigUint/BigInt operators work as expected.
#[multiversx_sc::module]
#[allow(clippy::redundant_clone)]
pub trait BigIntOperators {
    #[endpoint]
    fn add_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_int_big_uint(&self, a: BigInt, b: BigUint) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_uint_big_int(&self, a: BigUint, b: BigInt) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_int_big_uint_ref(&self, a: &BigInt, b: &BigUint) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_uint_big_int_ref(&self, a: &BigUint, b: &BigInt) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
    #[endpoint]
    fn add_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a + b
    }
    #[endpoint]
    fn add_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a + b
    }
    #[endpoint]
    fn sub_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        a - b
    }
    #[endpoint]
    fn sub_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a - b
    }
    #[endpoint]
    fn sub_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a - b
    }
    #[endpoint]
    fn sub_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a - b
    }
    #[endpoint]
    fn mul_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        a * b
    }
    #[endpoint]
    fn mul_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a * b
    }
    #[endpoint]
    fn mul_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a * b
    }
    #[endpoint]
    fn mul_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a * b
    }
    #[endpoint]
    fn div_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        a / b
    }
    #[endpoint]
    fn div_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a / b
    }
    #[endpoint]
    fn div_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a / b
    }
    #[endpoint]
    fn div_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a / b
    }
    #[endpoint]
    fn rem_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        a % b
    }
    #[endpoint]
    fn rem_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a % b
    }
    #[endpoint]
    fn rem_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a % b
    }
    #[endpoint]
    fn rem_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a % b
    }

    // assign version of all operators above
    #[endpoint]
    fn add_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn add_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn add_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn add_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r += b;
        r
    }
    #[endpoint]
    fn sub_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn sub_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn sub_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn sub_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r -= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn mul_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r *= b;
        r
    }
    #[endpoint]
    fn div_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        let mut r = a.clone();
        r /= b;
        r
    }
    #[endpoint]
    fn div_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut r = a.clone();
        r /= b;
        r
    }
    #[endpoint]
    fn div_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r /= b;
        r
    }
    #[endpoint]
    fn div_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r /= b;
        r
    }
    #[endpoint]
    fn rem_assign_big_int(&self, a: BigInt, b: BigInt) -> BigInt {
        let mut r = a.clone();
        r %= b;
        r
    }
    #[endpoint]
    fn rem_assign_big_int_ref(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let mut r = a.clone();
        r %= b;
        r
    }
    #[endpoint]
    fn rem_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r %= b;
        r
    }
    #[endpoint]
    fn rem_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r %= b;
        r
    }

    #[endpoint]
    fn bit_and_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a & b
    }
    #[endpoint]
    fn bit_and_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a & b
    }
    #[endpoint]
    fn bit_or_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a | b
    }
    #[endpoint]
    fn bit_or_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a | b
    }
    #[endpoint]
    fn bit_xor_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        a ^ b
    }
    #[endpoint]
    fn bit_xor_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a ^ b
    }

    #[endpoint]
    fn bit_and_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r &= b;
        r
    }
    #[endpoint]
    fn bit_and_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r &= b;
        r
    }
    #[endpoint]
    fn bit_or_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r |= b;
        r
    }
    #[endpoint]
    fn bit_or_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r |= b;
        r
    }
    #[endpoint]
    fn bit_xor_assign_big_uint(&self, a: BigUint, b: BigUint) -> BigUint {
        let mut r = a.clone();
        r ^= b;
        r
    }
    #[endpoint]
    fn bit_xor_assign_big_uint_ref(&self, a: &BigUint, b: &BigUint) -> BigUint {
        let mut r = a.clone();
        r ^= b;
        r
    }

    #[endpoint]
    fn shr_big_uint(&self, a: BigUint, b: usize) -> BigUint {
        a >> b
    }
    #[endpoint]
    fn shr_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
        a >> b
    }
    #[endpoint]
    fn shl_big_uint(&self, a: BigUint, b: usize) -> BigUint {
        a << b
    }
    #[endpoint]
    fn shl_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
        a << b
    }

    #[endpoint]
    fn shr_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint {
        let mut r = a.clone();
        r >>= b;
        r
    }
    #[endpoint]
    fn shr_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
        let mut r = a.clone();
        r >>= b;
        r
    }
    #[endpoint]
    fn shl_assign_big_uint(&self, a: BigUint, b: usize) -> BigUint {
        let mut r = a.clone();
        r <<= b;
        r
    }
    #[endpoint]
    fn shl_assign_big_uint_ref(&self, a: &BigUint, b: usize) -> BigUint {
        let mut r = a.clone();
        r <<= b;
        r
    }
}
