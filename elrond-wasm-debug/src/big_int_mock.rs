
use crate::big_uint_mock::*;

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use alloc::vec::Vec;

use num_bigint::BigInt;
use core::cmp::Ordering;

pub struct RustBigInt(pub num_bigint::BigInt);

impl RustBigInt {
    pub fn value(&self) -> &BigInt {
        &self.0
    }
}

impl From<RustBigUint> for RustBigInt {
    fn from(item: RustBigUint) -> Self {
        RustBigInt(item.0)
    }
}

impl From<i64> for RustBigInt {
    fn from(item: i64) -> Self {
        RustBigInt(item.into())
    }
}

impl From<i32> for RustBigInt {
    fn from(item: i32) -> Self {
        RustBigInt(item.into())
    }
}

impl From<BigInt> for RustBigInt {
    fn from(item: BigInt) -> Self {
        RustBigInt(item)
    }
}

impl Clone for RustBigInt {
    fn clone(&self) -> Self {
        RustBigInt(self.0.clone())
    }
}

impl Add for RustBigInt {
    type Output = RustBigInt;

    fn add(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 + other.0)
    }
}

impl AddAssign<RustBigInt> for RustBigInt {
    fn add_assign(&mut self, other: Self) {
        BigInt::add_assign(&mut self.0, other.0)
    }
}

impl AddAssign<&RustBigInt> for RustBigInt {
    fn add_assign(&mut self, other: &RustBigInt) {
        BigInt::add_assign(&mut self.0, &other.0)
    }
}

impl Sub for RustBigInt {
    type Output = RustBigInt;

    fn sub(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 - other.0)
    }
}

impl SubAssign<RustBigInt> for RustBigInt {
    fn sub_assign(&mut self, other: Self) {
        BigInt::sub_assign(&mut self.0, other.0)
    }
}

impl SubAssign<&RustBigInt> for RustBigInt {
    fn sub_assign(&mut self, other: &RustBigInt) {
        BigInt::sub_assign(&mut self.0, &other.0)
    }
}

impl Mul for RustBigInt {
    type Output = RustBigInt;

    fn mul(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 * other.0)
    }
}

impl MulAssign<RustBigInt> for RustBigInt {
    fn mul_assign(&mut self, other: Self) {
        BigInt::mul_assign(&mut self.0, other.0)
    }
}

impl MulAssign<&RustBigInt> for RustBigInt {
    fn mul_assign(&mut self, other: &RustBigInt) {
        BigInt::mul_assign(&mut self.0, &other.0)
    }
}

impl Div for RustBigInt {
    type Output = RustBigInt;

    fn div(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 / other.0)
    }
}

impl DivAssign<RustBigInt> for RustBigInt {
    fn div_assign(&mut self, other: Self) {
        BigInt::div_assign(&mut self.0, &other.0)
    }
}

impl DivAssign<&RustBigInt> for RustBigInt {
    fn div_assign(&mut self, other: &RustBigInt) {
        BigInt::div_assign(&mut self.0, &other.0)
    }
}

impl Rem for RustBigInt {
    type Output = RustBigInt;

    fn rem(self, other: RustBigInt) -> RustBigInt {
        RustBigInt(self.0 % other.0)
    }
}

impl RemAssign<RustBigInt> for RustBigInt {
    fn rem_assign(&mut self, other: Self) {
        BigInt::rem_assign(&mut self.0, &other.0)
    }
}

impl RemAssign<&RustBigInt> for RustBigInt {
    fn rem_assign(&mut self, other: &RustBigInt) {
        BigInt::rem_assign(&mut self.0, &other.0)
    }
}

impl PartialEq<Self> for RustBigInt {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl Eq for RustBigInt{}

impl PartialOrd<Self> for RustBigInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl Ord for RustBigInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl PartialEq<i64> for RustBigInt {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(&self.0, &BigInt::from(*other))
    }
}

impl PartialOrd<i64> for RustBigInt {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &BigInt::from(*other))
    }
}

impl elrond_wasm::BigIntApi<RustBigUint> for RustBigInt {
    
    fn abs(&self) -> RustBigUint {
        panic!("RustBigInt::abs not yet implemented")
    }

    fn phantom() -> Self {
        RustBigInt::from(0)
    }
}

impl RustBigInt {
    pub fn to_signed_bytes_be(&self) -> Vec<u8>{
        self.0.to_signed_bytes_be()
    }
}
