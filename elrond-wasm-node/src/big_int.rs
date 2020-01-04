

use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;
use core::ops::Mul;
use core::ops::MulAssign;

use alloc::vec::Vec;

extern {
    fn bigIntNew(value: i64) -> i32;

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

impl ArwenBigInt {
    pub fn from_i64(value: i64) -> ArwenBigInt {
        unsafe {
            ArwenBigInt{ handle: bigIntNew(value) }
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

impl MulAssign for ArwenBigInt {
    fn mul_assign(&mut self, other: Self) {
        unsafe {
            bigIntMul(self.handle, self.handle, other.handle);
        }
    }
}

impl elrond_wasm::BigIntApi for ArwenBigInt {
    fn compare(b1: &Self, b2: &Self) -> i32 {
        unsafe {
            bigIntCmp(b1.handle, b2.handle)
        }
    }

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
}
