
//use crate::big_int_mock::*;

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};
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

macro_rules! binary_operator {
    ($trait:ident, $method:ident) => {
        impl $trait for RustBigUint {
            type Output = RustBigUint;
        
            fn $method(self, other: RustBigUint) -> RustBigUint {
                RustBigUint((self.0).$method(other.0))
            }
        }

        impl<'a, 'b> $trait<&'b RustBigUint> for &'a RustBigUint {
            type Output = RustBigUint;
        
            fn $method(self, other: &RustBigUint) -> RustBigUint {
                RustBigUint(self.0.clone().$method(other.0.clone()))
            }
        }
    }
}

binary_operator!{Add, add}
binary_operator!{Mul, mul}
binary_operator!{Div, div}
binary_operator!{Rem, rem}

binary_operator!{BitAnd, bitand}
binary_operator!{BitOr, bitor}
binary_operator!{BitXor, bitxor}

fn check_sub_result(result: &BigInt) {
    if result.sign() == num_bigint::Sign::Minus {
        panic!(b"Cannot subtract because result would be negative")
    }
}

impl Sub for RustBigUint {
    type Output = RustBigUint;

    fn sub(self, other: RustBigUint) -> RustBigUint {
        let result = self.0 - other.0;
        check_sub_result(&result);
        RustBigUint(result)
    }
}

impl<'a, 'b> Sub<&'b RustBigUint> for &'a RustBigUint {
    type Output = RustBigUint;

    fn sub(self, other: &RustBigUint) -> RustBigUint {
        let result = self.0.clone().sub(other.0.clone());
        check_sub_result(&result);
        RustBigUint(result)
    }
}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident) => {
        impl $trait<RustBigUint> for RustBigUint {
            fn $method(&mut self, other: Self) {
                BigInt::$method(&mut self.0, other.0);
            }
        }
        
        impl $trait<&RustBigUint> for RustBigUint {
            fn $method(&mut self, other: &RustBigUint) {
                BigInt::$method(&mut self.0, &other.0);
            }
        }
    }
}

binary_assign_operator!{AddAssign, add_assign}
binary_assign_operator!{MulAssign, mul_assign}
binary_assign_operator!{DivAssign, div_assign}
binary_assign_operator!{RemAssign, rem_assign}

binary_assign_operator!{BitAndAssign, bitand_assign}
binary_assign_operator!{BitOrAssign,  bitor_assign}
binary_assign_operator!{BitXorAssign, bitxor_assign}

impl SubAssign<RustBigUint> for RustBigUint {
    fn sub_assign(&mut self, other: Self) {
        BigInt::sub_assign(&mut self.0, other.0);
        check_sub_result(&self.0);
    }
}

impl SubAssign<&RustBigUint> for RustBigUint {
    fn sub_assign(&mut self, other: &RustBigUint) {
        BigInt::sub_assign(&mut self.0, &other.0);
        check_sub_result(&self.0);
    }
}

macro_rules! shift_traits {
    ($shift_trait:ident, $method:ident, $api_func:ident) => {
        impl $shift_trait<i32> for RustBigUint {
            type Output = RustBigUint;

            fn $method(self, rhs: i32) -> RustBigUint {
                let result = $shift_trait::$method(self.0, rhs as usize);
                RustBigUint(result)
            }
        }
        
        impl<'a> $shift_trait<i32> for &'a RustBigUint {
            type Output = RustBigUint;

            fn $method(self, rhs: i32) -> RustBigUint {
                let result = $shift_trait::$method(&self.0, rhs as usize);
                RustBigUint(result)
            }
        }
    }
}

shift_traits!{Shl, shl, bigIntShl}
shift_traits!{Shr, shr, bigIntShr}

macro_rules! shift_assign_traits {
    ($shift_assign_trait:ident, $method:ident, $api_func:ident) => {
        impl $shift_assign_trait<i32> for RustBigUint {
            fn $method(&mut self, rhs: i32) {
                $shift_assign_trait::$method(&mut self.0, rhs as usize);
            }
        }
    }
}

shift_assign_traits!{ShlAssign, shl_assign, bigIntShl}
shift_assign_traits!{ShrAssign, shr_assign, bigIntShr}



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
