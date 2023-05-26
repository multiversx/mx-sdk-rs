use crate::{num_bigint, tx_mock::big_int_to_i64, DebugApi};
use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};
use multiversx_sc::{
    api::{BigIntApi, ErrorApiImpl, HandleTypeInfo, ManagedBufferApi, RawHandle},
    err_msg,
    types::heap::BoxedBytes,
};
use num_bigint::BigInt;
use num_traits::{pow, sign::Signed, Zero};
use std::convert::TryInto;

use super::{vh_error::VMHooksError, vh_managed_types::ManagedTypesSource};

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: RawHandle, x: RawHandle, y: RawHandle) {
            let bi_x = self.m_types_borrow().bi_get(x);
            let bi_y = self.m_types_borrow().bi_get(y);
            let result = bi_x.$rust_op_name(bi_y);
            self.m_types_borrow_mut().bi_overwrite(dest, result);
        }
    };
}

macro_rules! binary_bitwise_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: RawHandle, x: RawHandle, y: RawHandle) {
            let bi_x = self.m_types_borrow().bi_get(x);
            if bi_x.sign() == num_bigint::Sign::Minus {
                self.signal_vm_error(err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE);
            }
            let bi_y = self.m_types_borrow().bi_get(y);
            if bi_y.sign() == num_bigint::Sign::Minus {
                self.signal_vm_error(err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE);
            }
            let result = bi_x.$rust_op_name(bi_y);
            self.m_types_borrow_mut().bi_overwrite(dest, result);
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: RawHandle, x: RawHandle) {
            let bi_x = self.m_types_borrow().bi_get(x);
            let result = bi_x.$rust_op_name();
            self.m_types_borrow_mut().bi_overwrite(dest, result);
        }
    };
}

pub trait VMHooksBigInt: ManagedTypesSource + VMHooksError {
    // fn bi_new_from_big_int(
    //     &self,
    //     value: num_bigint::BigInt,
    // ) -> RawHandle {
    //     let mut managed_types = self.m_types_borrow_mut();
    //     managed_types.big_int_map.insert_new_handle(value)
    // }

    #[allow(dead_code)]
    fn bi_new(&self, value: i64) -> RawHandle {
        self.m_types_borrow_mut()
            .bi_new_from_big_int(num_bigint::BigInt::from(value))
    }

    fn bi_set_int64(&self, destination: RawHandle, value: i64) {
        self.m_types_borrow_mut()
            .bi_overwrite(destination, num_bigint::BigInt::from(value))
    }

    fn bi_unsigned_byte_length(&self, handle: RawHandle) -> usize {
        self.m_types_borrow().bi_get_unsigned_bytes(handle).len()
    }

    fn bi_get_unsigned_bytes(&self, handle: RawHandle) -> BoxedBytes {
        self.m_types_borrow().bi_get_unsigned_bytes(handle)
    }

    // fn bi_set_unsigned_bytes(&self, dest: RawHandle, bytes: &[u8]) {
    //     let result = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes);
    //     self.bi_overwrite(dest, result);
    // }

    // fn bi_signed_byte_length(&self, handle: RawHandle) -> usize {
    //     self.bi_get_signed_bytes(handle).len()
    // }

    // fn bi_get_signed_bytes(&self, handle: RawHandle) -> BoxedBytes {
    //     let bi = self.bi_get(handle);
    //     if bi.is_zero() {
    //         BoxedBytes::empty()
    //     } else {
    //         bi.to_signed_bytes_be().into()
    //     }
    // }

    // fn bi_set_signed_bytes(&self, dest: RawHandle, bytes: &[u8]) {
    //     let result = num_bigint::BigInt::from_signed_bytes_be(bytes);
    //     self.bi_overwrite(dest, result);
    // }

    // fn bi_to_i64(&self, handle: RawHandle) -> Option<i64> {
    //     let bi = self.bi_get(handle);
    //     big_int_to_i64(&bi)
    // }

    fn bi_is_int64(&self, destination_handle: RawHandle) -> i32 {
        if self
            .m_types_borrow()
            .bi_to_i64(destination_handle)
            .is_some()
        {
            1
        } else {
            0
        }
    }

    fn bi_get_int64(&self, destination_handle: RawHandle) -> i64 {
        self.m_types_borrow()
            .bi_to_i64(destination_handle)
            .unwrap_or_else(|| self.signal_vm_error(err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE))
    }

    binary_op_method! {bi_add, add}
    binary_op_method! {bi_sub, sub}
    binary_op_method! {bi_mul, mul}
    binary_op_method! {bi_t_div, div}
    binary_op_method! {bi_t_mod, rem}

    unary_op_method! {bi_abs, abs}
    unary_op_method! {bi_neg, neg}

    fn bi_sign(&self, x: RawHandle) -> i32 {
        let bi = self.m_types_borrow().bi_get(x);
        match bi.sign() {
            num_bigint::Sign::Minus => -1,
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => 1,
        }
    }

    fn bi_cmp(&self, x: RawHandle, y: RawHandle) -> i32 {
        let bi_x = self.m_types_borrow().bi_get(x);
        let bi_y = self.m_types_borrow().bi_get(y);
        match bi_x.cmp(&bi_y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    unary_op_method! {bi_sqrt, sqrt}

    fn bi_pow(&self, dest: RawHandle, x: RawHandle, y: RawHandle) {
        let bi_x = self.m_types_borrow().bi_get(x);
        let bi_y = self.m_types_borrow().bi_get(y);
        let exp = big_int_to_i64(&bi_y).unwrap().try_into().unwrap();
        let result = pow(bi_x, exp);
        self.m_types_borrow_mut().bi_overwrite(dest, result);
    }

    fn bi_log2(&self, x: RawHandle) -> i32 {
        let bi_x = self.m_types_borrow().bi_get(x);
        bi_x.bits() as i32 - 1
    }

    binary_bitwise_op_method! {bi_and, bitand}
    binary_bitwise_op_method! {bi_or, bitor}
    binary_bitwise_op_method! {bi_xor, bitxor}

    fn bi_shr(&self, dest: RawHandle, x: RawHandle, bits: usize) {
        let bi_x = self.m_types_borrow().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            self.signal_vm_error(err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE);
        }
        let result = bi_x.shr(bits);
        self.m_types_borrow_mut().bi_overwrite(dest, result);
    }

    fn bi_shl(&self, dest: RawHandle, x: RawHandle, bits: usize) {
        let bi_x = self.m_types_borrow().bi_get(x);
        if bi_x.sign() == num_bigint::Sign::Minus {
            self.signal_vm_error(err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE);
        }
        let result = bi_x.shl(bits);
        self.m_types_borrow_mut().bi_overwrite(dest, result);
    }

    // fn bi_to_string(&self, x: RawHandle, str_handle: RawHandle) {
    //     let s = {
    //         let bi_x = self.bi_get(x);
    //         bi_x.to_string()
    //     };
    //     self.mb_overwrite(str_handle, s.as_bytes());
    // }
}
