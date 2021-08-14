use crate::TxContext;

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use elrond_wasm::api::{BigIntApi, Handle};
use elrond_wasm::types::BoxedBytes;
use num_bigint::BigInt;
use num_traits::sign::Signed;
use num_traits::{pow, Zero};

use super::big_int_util::big_int_to_i64;

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Handle, x: Handle, y: Handle) {
            let mut tx_output = self.tx_output_cell.borrow_mut();
            let bi_x = tx_output.managed_types.big_int_map.get(x);
            let bi_y = tx_output.managed_types.big_int_map.get(y);
            let result = bi_x.$rust_op_name(bi_y);
            tx_output.managed_types.big_int_map.insert(dest, result);
        }
    };
}

impl BigIntApi for TxContext {
    fn bi_new(&self, value: i64) -> Handle {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .managed_types
            .big_int_map
            .insert_new_handle(BigInt::from(value))
    }

    fn bi_signed_byte_length(&self, handle: Handle) -> i32 {
        self.bi_get_signed_bytes(handle).len() as i32
    }

    fn bi_get_signed_bytes(&self, handle: Handle) -> BoxedBytes {
        let tx_output = self.tx_output_cell.borrow();
        let bi = tx_output.managed_types.big_int_map.get(handle);
        if bi.is_zero() {
            BoxedBytes::empty()
        } else {
            bi.to_signed_bytes_be().into()
        }
    }

    fn bi_set_signed_bytes(&self, dest: Handle, bytes: &[u8]) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let result = BigInt::from_signed_bytes_be(bytes);
        tx_output.managed_types.big_int_map.insert(dest, result);
    }

    fn bi_to_i64(&self, handle: Handle) -> Option<i64> {
        let tx_output = self.tx_output_cell.borrow();
        let bi = tx_output.managed_types.big_int_map.get(handle);
        big_int_to_i64(bi)
    }

    binary_op_method! {bi_add, add}
    binary_op_method! {bi_sub, sub}
    binary_op_method! {bi_mul, mul}
    binary_op_method! {bi_t_div, div}
    binary_op_method! {bi_t_mod, rem}

    fn bi_pow(&self, dest: Handle, x: Handle, y: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi_x = tx_output.managed_types.big_int_map.get(x);
        let bi_y = tx_output.managed_types.big_int_map.get(y);
        let exp = big_int_to_i64(bi_y).unwrap() as usize;
        let result = pow(bi_x.clone(), exp);
        tx_output.managed_types.big_int_map.insert(dest, result);
    }

    fn bi_abs(&self, dest: Handle, x: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi_x = tx_output.managed_types.big_int_map.get(x);
        let result = bi_x.abs();
        tx_output.managed_types.big_int_map.insert(dest, result);
    }

    fn bi_neg(&self, dest: Handle, x: Handle) {
        let mut tx_output = self.tx_output_cell.borrow_mut();
        let bi_x = tx_output.managed_types.big_int_map.get(x);
        let result = bi_x.neg();
        tx_output.managed_types.big_int_map.insert(dest, result);
    }

    fn bi_sign(&self, x: Handle) -> elrond_wasm::api::Sign {
        let tx_output = self.tx_output_cell.borrow();
        let bi = tx_output.managed_types.big_int_map.get(x);
        match bi.sign() {
            num_bigint::Sign::Minus => elrond_wasm::api::Sign::NoSign,
            num_bigint::Sign::NoSign => elrond_wasm::api::Sign::NoSign,
            num_bigint::Sign::Plus => elrond_wasm::api::Sign::Plus,
        }
    }

    fn bi_cmp(&self, x: Handle, y: Handle) -> Ordering {
        let tx_output = self.tx_output_cell.borrow();
        let bi_x = tx_output.managed_types.big_int_map.get(x);
        let bi_y = tx_output.managed_types.big_int_map.get(y);
        bi_x.cmp(bi_y)
    }
}
