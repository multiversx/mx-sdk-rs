use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

use alloc::string::String;
use alloc::vec::Vec;

use elrond_wasm::api::{BigIntApi, Handle, Sign};

extern "C" {
    fn bigIntNew(value: i64) -> i32;

    fn bigIntSignedByteLength(x: i32) -> i32;
    fn bigIntGetSignedBytes(reference: i32, byte_ptr: *mut u8) -> i32;
    fn bigIntSetSignedBytes(destination: i32, byte_ptr: *const u8, byte_len: i32);

    fn bigIntIsInt64(reference: i32) -> i32;
    fn bigIntGetInt64(reference: i32) -> i64;

    fn bigIntAdd(dest: i32, x: i32, y: i32);
    fn bigIntSub(dest: i32, x: i32, y: i32);
    fn bigIntMul(dest: i32, x: i32, y: i32);
    fn bigIntTDiv(dest: i32, x: i32, y: i32);
    fn bigIntTMod(dest: i32, x: i32, y: i32);

    fn bigIntPow(dest: i32, x: i32, y: i32);
    fn bigIntAbs(dest: i32, x: i32);
    fn bigIntNeg(dest: i32, x: i32);
    fn bigIntSign(x: i32) -> i32;
    fn bigIntCmp(x: i32, y: i32) -> i32;
}

macro_rules! binary_op_wrapper {
	($method_name, $hook_name) => {
		fn $method_name(&self, dest: Handle, x: Handle, y: Handle) {
			unsafe {
				$hook_name(dest, x, y);
			}
		}
	};
}

impl BigIntApi for ArwenApiImpl {
    fn new(&self, value: i64) -> Handle {
        unsafe { bigIntNew(value) }
    }

    fn signed_byte_length(&self, x: Handle) -> Handle {
        unsafe { bigIntSignedByteLength(x) }
    }

    fn get_signed_bytes(&self, reference: Handle) -> BoxedBytes {
        unsafe {
            let byte_len = bigIntSignedByteLength(self.handle);
            let mut vec = vec![0u8; byte_len as usize];
            bigIntGetSignedBytes(self.handle, vec.as_mut_ptr());
            vec
        }
    }

    fn set_signed_bytes(&self, destination: Handle, bytes: &[u8]) {
        unsafe { bigIntSetSignedBytes(destination, bytes) }
    }

    fn is_int64(&self, reference: Handle) -> Handle {
        unsafe { bigIntIsInt64(reference) }
    }

    fn get_int64(&self, reference: Handle) -> i64 {
        unsafe { bigIntGetInt64(reference) }
    }

	binary_op_wrapper!{add, bigIntAdd}
	binary_op_wrapper!{sub, bigIntSub}
	binary_op_wrapper!{mul, bigIntMul}
	binary_op_wrapper!{t_div, bigIntTDiv}
	binary_op_wrapper!{t_mod, bigIntTMod}
	binary_op_wrapper!{pow, bigIntPow}

    fn abs(&self, dest: Handle, x: Handle) {
        unsafe {
            bigIntAbs(dest, x);
        }
    }

    fn neg(&self, dest: Handle, x: Handle) {
        unsafe {
            bigIntNeg(dest, x);
        }
    }

    fn sign(&self, x: Handle) -> i32 {
        unsafe { bigIntSign(x) }
    }

    fn cmp(&self, x: Handle, y: Handle) -> i32 {
        unsafe { bigIntCmp(x, y) }
    }
}
