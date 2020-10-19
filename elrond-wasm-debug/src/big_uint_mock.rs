
//use crate::big_int_mock::*;

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};
use alloc::vec::Vec;
use elrond_wasm::BigUintApi;

use num_bigint::{Sign, BigUint, BigInt};
use core::cmp::Ordering;

#[derive(Debug)]
pub struct RustBigUint(pub num_bigint::BigInt);

impl RustBigUint {
    pub fn value(&self) -> BigUint {
        self.0.to_biguint().unwrap()
    }
}

impl From<u64> for RustBigUint {
    fn from(item: u64) -> Self {
        RustBigUint((item as i64).into())
    }
}

impl From<u32> for RustBigUint {
    fn from(item: u32) -> Self {
        RustBigUint((item as i32).into())
    }
}

impl From<usize> for RustBigUint {
    fn from(item: usize) -> Self {
        RustBigUint((item as i32).into())
    }
}

impl From<BigInt> for RustBigUint {
    fn from(item: BigInt) -> Self {
        RustBigUint(item)
    }
}

impl From<BigUint> for RustBigUint {
    fn from(item: BigUint) -> Self {
        RustBigUint(BigInt::from_biguint(Sign::Plus, item))
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
    ($shift_trait:ident, $method:ident) => {
        impl $shift_trait<usize> for RustBigUint {
            type Output = RustBigUint;

            fn $method(self, rhs: usize) -> RustBigUint {
                let result = $shift_trait::$method(self.0, rhs);
                RustBigUint(result)
            }
        }
        
        impl<'a> $shift_trait<usize> for &'a RustBigUint {
            type Output = RustBigUint;

            fn $method(self, rhs: usize) -> RustBigUint {
                let result = $shift_trait::$method(&self.0, rhs);
                RustBigUint(result)
            }
        }
    }
}

shift_traits!{Shl, shl}
shift_traits!{Shr, shr}

macro_rules! shift_assign_traits {
    ($shift_assign_trait:ident, $method:ident) => {
        impl $shift_assign_trait<usize> for RustBigUint {
            fn $method(&mut self, rhs: usize) {
                $shift_assign_trait::$method(&mut self.0, rhs);
            }
        }
    }
}

shift_assign_traits!{ShlAssign, shl_assign}
shift_assign_traits!{ShrAssign, shr_assign}



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

impl PartialEq<u64> for RustBigUint {
    #[inline]
    fn eq(&self, other: &u64) -> bool {
        PartialEq::eq(&self.0, &BigInt::from(*other as i64))
    }
}

impl PartialOrd<u64> for RustBigUint {
    #[inline]
    fn partial_cmp(&self, other: &u64) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &BigInt::from(*other as i64))
    }
}

use elrond_wasm::elrond_codec::*;

impl NestedEncode for RustBigUint {
    const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

    fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
        let bytes = self.to_bytes_be();
        f(&bytes);
        Ok(())
    }
    
    fn dep_encode_to<O: NestedOutputBuffer>(&self, dest: &mut O) -> Result<(), EncodeError> {
        let bytes = self.to_bytes_be();
        bytes.as_slice().dep_encode_to(dest)
    }
}

impl TopEncode for RustBigUint {
	fn top_encode<B: TopEncodeBuffer, O: TopEncodeOutput<B>>(&self, output: O) -> Result<(), EncodeError> {
		self.to_bytes_be().top_encode(output)
	}
}

impl NestedDecode for RustBigUint {
    const TYPE_INFO: TypeInfo = TypeInfo::BigUint;

    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        let bytes = input.read_slice(size)?;
        Ok(RustBigUint::from_bytes_be(bytes))
    }
}

impl TopDecode for RustBigUint {
	fn top_decode<I: TopDecodeInput>(mut input: I) -> Result<Self, DecodeError> {
        Ok(RustBigUint::from_bytes_be(input.get_slice_u8()))
    }
}

impl elrond_wasm::BigUintApi for RustBigUint {

    fn byte_length(&self) -> i32 {
        panic!("byte_length not yet implemented")
    }

    fn copy_to_slice_big_endian(&self, _slice: &mut [u8]) -> i32 {
        panic!("copy_to_slice not yet implemented")
    }

    fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
        if self.0.sign() == Sign::Plus {
            let (_, bytes) = self.0.to_bytes_be();
            let offset = 32 - bytes.len();
            target[offset..].clone_from_slice(&bytes[..]);
        }
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        if self.0.sign() == Sign::NoSign {
            Vec::new()
        } else {
            let (_, be) = self.0.to_bytes_be();
            be
        }
    }

    fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Option<Vec<u8>> {
        let (_, bytes_be) = self.0.to_bytes_be();
        match bytes_be.len().cmp(&nr_bytes) {
            Ordering::Greater => None,
            Ordering::Equal => Some(bytes_be),
            Ordering::Less => {
                let mut res = vec![0u8; nr_bytes];
                let offset = nr_bytes - bytes_be.len();
                res[offset..].clone_from_slice(&bytes_be[..]);
                Some(res)
            }
        }
    }

    fn from_bytes_be(bytes: &[u8]) -> Self {
        let bi = BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes);
        bi.into()
    }
}
