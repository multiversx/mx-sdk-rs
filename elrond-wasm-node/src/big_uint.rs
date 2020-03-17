
//use crate::big_int::*;

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::cmp::Ordering;

use alloc::vec::Vec;

extern {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntUnsignedByteLength(x: i32) -> i32;
    fn bigIntGetUnsignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetUnsignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);
    
    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntTDiv(dest: i32, x: i32, y: i32);
    fn bigIntTMod(dest: i32, x: i32, y: i32);
    fn bigIntCmp(x: i32, y: i32) -> i32;
}

pub struct ArwenBigUint {
    pub handle: i32 // TODO: fix visibility
}

impl From<i64> for ArwenBigUint {
    fn from(item: i64) -> Self {
        unsafe {
            ArwenBigUint{ handle: bigIntNew(item) }
        }
    }
}

impl From<i32> for ArwenBigUint {
    fn from(item: i32) -> Self {
        unsafe {
            ArwenBigUint{ handle: bigIntNew(item.into()) }
        }
    }
}

impl ArwenBigUint {
    pub fn from_i64(value: i64) -> ArwenBigUint {
        unsafe {
            ArwenBigUint{ handle: bigIntNew(value) }
        }
    }
}

impl Clone for ArwenBigUint {
    fn clone(&self) -> Self {
        unsafe {
            let clone_handle = bigIntNew(0);
            bigIntAdd(clone_handle, clone_handle, self.handle);
            ArwenBigUint {handle: clone_handle}
        }        
    }
}

impl Add for ArwenBigUint {
    type Output = ArwenBigUint;

    fn add(self, other: ArwenBigUint) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntAdd(result, self.handle, other.handle);
            ArwenBigUint {handle: result}
        }
    }
}

impl AddAssign<ArwenBigUint> for ArwenBigUint {
    fn add_assign(&mut self, other: Self) {
        unsafe {
            bigIntAdd(self.handle, self.handle, other.handle);
        }
    }
}

impl AddAssign<&ArwenBigUint> for ArwenBigUint {
    fn add_assign(&mut self, other: &ArwenBigUint) {
        unsafe {
            bigIntAdd(self.handle, self.handle, other.handle);
        }
    }
}

impl Sub for ArwenBigUint {
    type Output = ArwenBigUint;

    fn sub(self, other: ArwenBigUint) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntSub(result, self.handle, other.handle);
            ArwenBigUint {handle: result}
        }
    }
}

impl SubAssign<ArwenBigUint> for ArwenBigUint {
    fn sub_assign(&mut self, other: Self) {
        unsafe {
            bigIntSub(self.handle, self.handle, other.handle);
        }
    }
}

impl SubAssign<&ArwenBigUint> for ArwenBigUint {
    fn sub_assign(&mut self, other: &ArwenBigUint) {
        unsafe {
            bigIntSub(self.handle, self.handle, other.handle);
        }
    }
}

impl Mul for ArwenBigUint {
    type Output = ArwenBigUint;

    fn mul(self, other: ArwenBigUint) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntMul(result, self.handle, other.handle);
            ArwenBigUint {handle: result}
        }
    }
}

impl MulAssign<ArwenBigUint> for ArwenBigUint {
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
        }
    }
}

impl MulAssign<&ArwenBigUint> for ArwenBigUint {
    fn mul_assign(&mut self, other: &ArwenBigUint) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
        }
    }
}

impl Div for ArwenBigUint {
    type Output = ArwenBigUint;

    fn div(self, other: ArwenBigUint) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntTDiv(result, self.handle, other.handle);
            ArwenBigUint {handle: result}
        }
    }
}

impl DivAssign<ArwenBigUint> for ArwenBigUint {
    fn div_assign(&mut self, other: Self) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl DivAssign<&ArwenBigUint> for ArwenBigUint {
    fn div_assign(&mut self, other: &ArwenBigUint) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl Rem for ArwenBigUint {
    type Output = ArwenBigUint;

    fn rem(self, other: ArwenBigUint) -> ArwenBigUint {
        unsafe {
            let result = bigIntNew(0);
            bigIntTDiv(result, self.handle, other.handle);
            ArwenBigUint {handle: result}
        }
    }
}

impl RemAssign<ArwenBigUint> for ArwenBigUint {
    fn rem_assign(&mut self, other: Self) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl RemAssign<&ArwenBigUint> for ArwenBigUint {
    fn rem_assign(&mut self, other: &ArwenBigUint) {
        unsafe {
            bigIntTDiv(self.handle, self.handle, other.handle);
        }
    }
}

impl PartialEq for ArwenBigUint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let arwen_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
        arwen_cmp == 0
    }
}

impl Eq for ArwenBigUint{}

impl PartialOrd for ArwenBigUint {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArwenBigUint {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let arwen_cmp = unsafe { bigIntCmp(self.handle, other.handle) };
        if arwen_cmp == 0 {
            Ordering::Equal
        } else if arwen_cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq<i64> for ArwenBigUint {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(self, &ArwenBigUint::from(*other))
    }
}

impl PartialOrd<i64> for ArwenBigUint {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        PartialOrd::partial_cmp(self, &ArwenBigUint::from(*other))
    }
}

impl elrond_wasm::BigUintApi for ArwenBigUint {
    #[inline]
    fn byte_length(&self) -> i32 {
        unsafe { bigIntUnsignedByteLength(self.handle) }
    }

    fn copy_to_slice_big_endian(&self, slice: &mut [u8]) -> i32 {
        unsafe {
            let byte_len = bigIntGetUnsignedBytes(self.handle, slice.as_mut_ptr());
            byte_len
        }
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntUnsignedByteLength(self.handle);
            let mut vec = vec![0u8; byte_len as usize];
            bigIntGetUnsignedBytes(self.handle, vec.as_mut_ptr());
            vec
        }
    }

    fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntUnsignedByteLength(self.handle) as usize;
            if byte_len > nr_bytes {
                panic!();
            }
            let mut vec = vec![0u8; nr_bytes];
            if byte_len > 0 {
                bigIntGetUnsignedBytes(self.handle, &mut vec[nr_bytes - byte_len]);
            }
            vec
        }
    }

    #[inline]
    fn phantom() -> Self {
        ArwenBigUint{ handle: -1 }
    }
}
