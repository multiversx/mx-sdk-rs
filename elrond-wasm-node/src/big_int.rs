

use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ops::Mul;
use core::ops::MulAssign;
use core::cmp::Ordering;

use alloc::vec::Vec;

extern {
    fn bigIntNew(value: i64) -> i32;
    fn bigIntClone(reference: i32) -> i32;
    fn bigIntDestruct(reference: i32);

    fn bigIntByteLength(x: i32) -> i32;
    fn bigIntGetBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
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
        let new_handle = unsafe {
            bigIntClone(self.handle)
        };
        ArwenBigInt {handle: new_handle}
    }
}

impl Drop for ArwenBigInt {
    fn drop(&mut self) {
        unsafe { bigIntDestruct(self.handle) };
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

impl MulAssign for ArwenBigInt {
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
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

impl PartialOrd for ArwenBigInt {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl elrond_wasm::BigIntApi for ArwenBigInt {
    #[inline]
    fn byte_length(&self) -> i32 {
        unsafe { bigIntByteLength(self.handle) }
    }

    fn copy_to_slice(&self, slice: &mut [u8]) -> i32 {
        unsafe {
            let byte_len = bigIntGetBytes(self.handle, slice.as_mut_ptr());
            byte_len
        }
    }

    fn get_bytes_big_endian(&self) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntByteLength(self.handle);
            let mut vec = vec![0u8; byte_len as usize];
            bigIntGetBytes(self.handle, vec.as_mut_ptr());
            vec
        }
    }

    fn get_bytes_big_endian_pad_right(&self, nr_bytes: usize) -> Vec<u8> {
        unsafe {
            let byte_len = bigIntByteLength(self.handle) as usize;
            if byte_len > nr_bytes {
                panic!();
            }
            let mut vec = vec![0u8; nr_bytes];
            if byte_len > 0 {
                bigIntGetBytes(self.handle, &mut vec[nr_bytes - byte_len]);
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
