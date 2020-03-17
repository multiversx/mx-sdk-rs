

use crate::big_uint::*;

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::cmp::Ordering;

use alloc::vec::Vec;

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

impl Add for ArwenBigInt {
    type Output = ArwenBigInt;

    fn add(self, other: ArwenBigInt) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntAdd(result, self.handle, other.handle);
            ArwenBigInt {handle: result}
        }
    }
}

impl AddAssign<ArwenBigInt> for ArwenBigInt {
    fn add_assign(&mut self, other: Self) {
        unsafe {
            bigIntAdd(self.handle, self.handle, other.handle);
        }
    }
}

impl AddAssign<&ArwenBigInt> for ArwenBigInt {
    fn add_assign(&mut self, other: &ArwenBigInt) {
        unsafe {
            bigIntAdd(self.handle, self.handle, other.handle);
        }
    }
}

impl Sub for ArwenBigInt {
    type Output = ArwenBigInt;

    fn sub(self, other: ArwenBigInt) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntSub(result, self.handle, other.handle);
            ArwenBigInt {handle: result}
        }
    }
}

impl SubAssign<ArwenBigInt> for ArwenBigInt {
    fn sub_assign(&mut self, other: Self) {
        unsafe {
            bigIntSub(self.handle, self.handle, other.handle);
        }
    }
}

impl SubAssign<&ArwenBigInt> for ArwenBigInt {
    fn sub_assign(&mut self, other: &ArwenBigInt) {
        unsafe {
            bigIntSub(self.handle, self.handle, other.handle);
        }
    }
}

impl Mul for ArwenBigInt {
    type Output = ArwenBigInt;

    fn mul(self, other: ArwenBigInt) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntMul(result, self.handle, other.handle);
            ArwenBigInt {handle: result}
        }
    }
}

impl MulAssign<ArwenBigInt> for ArwenBigInt {
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
        }
    }
}

impl MulAssign<&ArwenBigInt> for ArwenBigInt {
    fn mul_assign(&mut self, other: &ArwenBigInt) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
        }
    }
}

impl Div for ArwenBigInt {
    type Output = ArwenBigInt;

    fn div(self, other: ArwenBigInt) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntTDiv(result, self.handle, other.handle);
            ArwenBigInt {handle: result}
        }
    }
}

impl DivAssign<ArwenBigInt> for ArwenBigInt {
    fn div_assign(&mut self, other: Self) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl DivAssign<&ArwenBigInt> for ArwenBigInt {
    fn div_assign(&mut self, other: &ArwenBigInt) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl Rem for ArwenBigInt {
    type Output = ArwenBigInt;

    fn rem(self, other: ArwenBigInt) -> ArwenBigInt {
        unsafe {
            let result = bigIntNew(0);
            bigIntTDiv(result, self.handle, other.handle);
            ArwenBigInt {handle: result}
        }
    }
}

impl RemAssign<ArwenBigInt> for ArwenBigInt {
    fn rem_assign(&mut self, other: Self) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl RemAssign<&ArwenBigInt> for ArwenBigInt {
    fn rem_assign(&mut self, other: &ArwenBigInt) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

fn ordering(i: i32) -> Ordering {
    if i == 0 {
        Ordering::Equal
    } else if i > 0 {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

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
        ordering(arwen_cmp)
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
        Some(ordering(arwen_cmp))
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

impl elrond_wasm::BigIntApi<ArwenBigUint> for ArwenBigInt {

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
            if s == 0 {
                Sign::NoSign
            } else if s > 0 {
                Sign::Plus
            } else {
                Sign::Minus
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

    #[inline]
    fn phantom() -> Self {
        ArwenBigInt{ handle: -1 }
    }
}
