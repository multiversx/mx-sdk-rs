

use crate::big_uint::*;

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::cmp::Ordering;

use alloc::vec::Vec;

use elrond_wasm::BigIntApi;
use elrond_wasm::Sign;

extern {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntSignedByteLength(x: i32) -> i32;
    fn bigIntGetSignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetSignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntTDiv(dest: i32, x: i32, y: i32);
    fn bigIntTMod(dest: i32, x: i32, y: i32);

    fn bigIntAbs(dest: i32, x: i32);
    fn bigIntNeg(dest: i32, x: i32);
    fn bigIntSign(x: i32) -> i32;
    fn bigIntCmp(x: i32, y: i32) -> i32;
}

pub struct ArwenBigInt {
    pub handle: i32 // TODO: fix visibility
}

impl From<ArwenBigUint> for ArwenBigInt {
    #[inline]
    fn from(item: ArwenBigUint) -> Self {
        ArwenBigInt{ handle: item.handle }
    }
}

impl From<i64> for ArwenBigInt {
    fn from(item: i64) -> Self {
        unsafe {
            ArwenBigInt{ handle: bigIntNew(item) }
        }
    }
}

impl From<i32> for ArwenBigInt {
    fn from(item: i32) -> Self {
        unsafe {
            ArwenBigInt{ handle: bigIntNew(item.into()) }
        }
    }
}

impl ArwenBigInt {
    pub fn from_i64(value: i64) -> ArwenBigInt {
        unsafe {
            ArwenBigInt{ handle: bigIntNew(value) }
        }
    }
}

impl Clone for ArwenBigInt {
    fn clone(&self) -> Self {
        unsafe {
            let clone_handle = bigIntNew(0);
            bigIntAdd(clone_handle, clone_handle, self.handle);
            ArwenBigInt {handle: clone_handle}
        }        
    }
}

macro_rules! binary_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl $trait for ArwenBigInt {
            type Output = ArwenBigInt;
        
            fn $method(self, other: ArwenBigInt) -> ArwenBigInt {
                unsafe {
                    let result = bigIntNew(0);
                    $api_func(result, self.handle, other.handle);
                    ArwenBigInt {handle: result}
                }
            }
        }

        impl<'a, 'b> $trait<&'b ArwenBigInt> for &'a ArwenBigInt {
            type Output = ArwenBigInt;
        
            fn $method(self, other: &ArwenBigInt) -> ArwenBigInt {
                unsafe {
                    let result = bigIntNew(0);
                    $api_func(result, self.handle, other.handle);
                    ArwenBigInt {handle: result}
                }
            }
        }
    }
}

binary_operator!{Add, add, bigIntAdd}
binary_operator!{Sub, sub, bigIntSub}
binary_operator!{Mul, mul, bigIntMul}
binary_operator!{Div, div, bigIntTDiv}
binary_operator!{Rem, rem, bigIntTMod}

macro_rules! binary_assign_operator {
    ($trait:ident, $method:ident, $api_func:ident) => {
        impl $trait<ArwenBigInt> for ArwenBigInt {
            #[inline]
            fn $method(&mut self, other: Self) {
                unsafe {
                    $api_func(self.handle, self.handle, other.handle);
                }
            }
        }
        
        impl $trait<&ArwenBigInt> for ArwenBigInt {
            #[inline]
            fn $method(&mut self, other: &ArwenBigInt) {
                unsafe {
                    $api_func(self.handle, self.handle, other.handle);
                }
            }
        }
    }
}

binary_assign_operator!{AddAssign, add_assign, bigIntAdd}
binary_assign_operator!{SubAssign, sub_assign, bigIntSub}
binary_assign_operator!{MulAssign, mul_assign, bigIntMul}
binary_assign_operator!{DivAssign, div_assign, bigIntTDiv}
binary_assign_operator!{RemAssign, rem_assign, bigIntTMod}

impl PartialEq for ArwenBigInt {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let arwen_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
        arwen_cmp == 0
    }
}

impl Eq for ArwenBigInt{}

impl PartialOrd for ArwenBigInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArwenBigInt {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let arwen_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
        arwen_cmp.cmp(&0)
    }
}

fn arwen_cmp_i64(bi: &ArwenBigInt, other: i64) -> i32 {
    unsafe {
        if other == 0 {
            bigIntSign(bi.handle)
        } else {
            bigIntCmp(bi.handle, bigIntNew(other))
        }
    }
}

impl PartialEq<i64> for ArwenBigInt {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        arwen_cmp_i64(&self, *other) == 0
    }
}

impl PartialOrd<i64> for ArwenBigInt {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        let arwen_cmp = arwen_cmp_i64(&self, *other);
        Some(arwen_cmp.cmp(&0))
    }
}

impl Neg for ArwenBigInt {
    type Output = ArwenBigInt;

    fn neg(self) -> Self::Output {
        unsafe {
            let result = bigIntNew(0);
            bigIntNeg(result, self.handle);
            ArwenBigInt {handle: result}
        }
    }
}

use elrond_wasm::elrond_codec::*;

impl Encode for ArwenBigInt {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
        let bytes = self.to_signed_bytes_be();
        f(&bytes);
        Ok(())
    }
    
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        // TODO: vector allocation can be avoided by writing directly to dest
        let bytes = self.to_signed_bytes_be();
        bytes.as_slice().dep_encode_to(dest)
    }
}

impl NestedDecode for ArwenBigInt {
    const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        let bytes = input.read_slice(size)?;
        Ok(ArwenBigInt::from_signed_bytes_be(bytes))
    }
}

impl TopDecode for ArwenBigInt {
	fn top_decode<I: TopDecodeInput>(mut input: I) -> Result<Self, DecodeError> {
        Ok(ArwenBigInt::from_signed_bytes_be(input.get_slice_u8()))
    }
}

impl BigIntApi<ArwenBigUint> for ArwenBigInt {

    fn abs_uint(&self) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntAbs(result, self.handle);
            ArwenBigUint {handle: result}
        }
    }

    fn sign(&self) -> Sign {
        unsafe {
            let s = bigIntSign(self.handle);
            match s.cmp(&0) {
                Ordering::Greater => Sign::Plus,
                Ordering::Equal => Sign::NoSign,
                Ordering::Less => Sign::Minus,
            }
        }
    }

    fn to_signed_bytes_be(&self) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntSignedByteLength(self.handle);
            let mut vec = vec![0u8; byte_len as usize];
            bigIntGetSignedBytes(self.handle, vec.as_mut_ptr());
            vec
        }
    }

    fn from_signed_bytes_be(bytes: &[u8]) -> Self {
        unsafe {
            let handle = bigIntNew(0);
            bigIntSetSignedBytes(handle, bytes.as_ptr(), bytes.len() as i32);
            ArwenBigInt{ handle }
        }
    }
}
