
//use crate::big_int_mock::*;

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

use alloc::vec::Vec;

use num_bigint::BigInt;
use core::cmp::Ordering;

pub struct RustBigUint(pub num_bigint::BigInt);

impl RustBigUint {
    pub fn value(&self) -> &BigInt {
        &self.0
    }
}

impl From<i64> for RustBigUint {
    fn from(item: i64) -> Self {
        RustBigUint(item.into())
    }
}

impl From<i32> for RustBigUint {
    fn from(item: i32) -> Self {
        RustBigUint(item.into())
    }
}

impl From<BigInt> for RustBigUint {
    fn from(item: BigInt) -> Self {
        RustBigUint(item)
    }
}

impl Clone for RustBigUint {
    fn clone(&self) -> Self {
        RustBigUint(self.0.clone())
    }
}

impl Add for RustBigUint {
    type Output = RustBigUint;

    fn add(self, other: RustBigUint) -> RustBigUint {
        RustBigUint(self.0 + other.0)
    }
}

impl AddAssign<RustBigUint> for RustBigUint {
    fn add_assign(&mut self, other: Self) {
        BigInt::add_assign(&mut self.0, other.0)
    }
}

impl AddAssign<&RustBigUint> for RustBigUint {
    fn add_assign(&mut self, other: &RustBigUint) {
        BigInt::add_assign(&mut self.0, &other.0)
    }
}

impl Sub for RustBigUint {
    type Output = RustBigUint;

    fn sub(self, other: RustBigUint) -> RustBigUint {
        RustBigUint(self.0 - other.0)
    }
}

impl SubAssign<RustBigUint> for RustBigUint {
    fn sub_assign(&mut self, other: Self) {
        BigInt::sub_assign(&mut self.0, other.0)
    }
}

impl SubAssign<&RustBigUint> for RustBigUint {
    fn sub_assign(&mut self, other: &RustBigUint) {
        BigInt::sub_assign(&mut self.0, &other.0)
    }
}

impl Mul for RustBigUint {
    type Output = RustBigUint;

    fn mul(self, other: RustBigUint) -> RustBigUint {
        RustBigUint(self.0 * other.0)
    }
}

impl MulAssign<RustBigUint> for RustBigUint {
    fn mul_assign(&mut self, other: Self) {
        BigInt::mul_assign(&mut self.0, other.0)
    }
}

impl MulAssign<&RustBigUint> for RustBigUint {
    fn mul_assign(&mut self, other: &RustBigUint) {
        BigInt::mul_assign(&mut self.0, &other.0)
    }
}

impl Div for RustBigUint {
    type Output = RustBigUint;

    fn div(self, other: RustBigUint) -> RustBigUint {
        RustBigUint(self.0 / other.0)
    }
}

impl DivAssign<RustBigUint> for RustBigUint {
    fn div_assign(&mut self, other: Self) {
        BigInt::div_assign(&mut self.0, &other.0)
    }
}

impl DivAssign<&RustBigUint> for RustBigUint {
    fn div_assign(&mut self, other: &RustBigUint) {
        BigInt::div_assign(&mut self.0, &other.0)
    }
}

impl Rem for RustBigUint {
    type Output = RustBigUint;

    fn rem(self, other: RustBigUint) -> RustBigUint {
        RustBigUint(self.0 % other.0)
    }
}

impl RemAssign<RustBigUint> for RustBigUint {
    fn rem_assign(&mut self, other: Self) {
        BigInt::rem_assign(&mut self.0, &other.0)
    }
}

impl RemAssign<&RustBigUint> for RustBigUint {
    fn rem_assign(&mut self, other: &RustBigUint) {
        BigInt::rem_assign(&mut self.0, &other.0)
    }
}

impl PartialEq<Self> for RustBigUint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.0, &other.0)
    }
}

impl Eq for RustBigUint{}

impl PartialOrd<Self> for RustBigUint {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl Ord for RustBigUint {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

impl PartialEq<i64> for RustBigUint {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(&self.0, &BigInt::from(*other))
    }
}

impl PartialOrd<i64> for RustBigUint {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &BigInt::from(*other))
    }
}

impl elrond_wasm::BigUintApi for RustBigUint {

    fn byte_length(&self) -> i32 {
        panic!("byte_length not yet implemented")
    }

    fn copy_to_slice_big_endian(&self, _slice: &mut [u8]) -> i32 {
        panic!("copy_to_slice not yet implemented")
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        let (_, be) = self.0.to_bytes_be();
        be
    }

    fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Vec<u8> {
        let (_, bytes_be) = self.0.to_bytes_be();
        if bytes_be.len() > nr_bytes {
            panic!("Number doesn't fit requested bytes");
        } else if bytes_be.len() == nr_bytes {
            bytes_be
        } else {
            let mut res = vec![0u8; nr_bytes];
            let offset = nr_bytes - bytes_be.len();
            for i in 0..bytes_be.len()-1 {
                res[offset+i] = bytes_be[i];
            }
            res
        }
    }

    fn phantom() -> Self {
        RustBigUint::from(0)
    }
}
