

use core::ops::{Add, Sub, Mul, Div, Rem};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::cmp::Ordering;

use alloc::vec::Vec;

extern {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntUnsignedByteLength(x: i32) -> i32;
    fn bigIntGetUnsignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntGetSignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetUnsignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);
    fn bigIntSetSignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntTDiv(dest: i32, x: i32, y: i32);
    fn bigIntTMod(dest: i32, x: i32, y: i32);
    fn bigIntCmp(x: i32, y: i32) -> i32;
}

pub struct ArwenBigInt {
    pub handle: i32 // TODO: fix visibility
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
        if arwen_cmp == 0 {
            Ordering::Equal
        } else if arwen_cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq<i64> for ArwenBigInt {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        PartialEq::eq(self, &ArwenBigInt::from(*other))
    }
}

impl PartialOrd<i64> for ArwenBigInt {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        PartialOrd::partial_cmp(self, &ArwenBigInt::from(*other))
    }
}

impl elrond_wasm::BigIntApi for ArwenBigInt {
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

    fn to_bytes_big_endian(&self) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntUnsignedByteLength(self.handle);
            let mut vec = vec![0u8; byte_len as usize];
            bigIntGetUnsignedBytes(self.handle, vec.as_mut_ptr());
            vec
        }
    }

    fn to_bytes_big_endian_pad_right(&self, nr_bytes: usize) -> Vec<u8> {
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
        ArwenBigInt{ handle: -1 }
    }
}

pub struct ArwenBigUint {
    pub handle: i32 // TODO: make private, only finish method uses it
}

impl From<ArwenBigInt> for ArwenBigUint {
    #[inline]
    fn from(item: ArwenBigInt) -> Self {
        ArwenBigUint{ handle: item.handle }
    }
}

impl elrond_wasm::BigUintApi<ArwenBigInt> for ArwenBigUint {
    #[inline]
    fn into_signed(self) -> ArwenBigInt {
        ArwenBigInt{ handle: self.handle }
    }

    #[inline]
    fn phantom() -> Self {
        ArwenBigUint{ handle: -1 }
    }
}
