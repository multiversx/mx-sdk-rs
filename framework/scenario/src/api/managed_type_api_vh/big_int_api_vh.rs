use core::cmp::Ordering;

use multiversx_chain_vm::mem_conv;
use multiversx_sc::{
    api::{BigIntApi, HandleConstraints, Sign},
    types::BoxedBytes,
};

use crate::api::VMHooksBackend;

macro_rules! binary_op_method {
    ($api_method_name:ident, $vh_method_name:ident) => {
        fn $api_method_name(
            &self,
            dest: Self::BigIntHandle,
            x: Self::BigIntHandle,
            y: Self::BigIntHandle,
        ) {
            self.with_vm_hooks(|vh| {
                vh.$vh_method_name(
                    dest.get_raw_handle(),
                    x.get_raw_handle(),
                    y.get_raw_handle(),
                )
            });
        }
    };
}

macro_rules! unary_op_method {
    ($api_method_name:ident, $vh_method_name:ident) => {
        fn $api_method_name(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle) {
            self.with_vm_hooks(|vh| vh.$vh_method_name(dest.get_raw_handle(), x.get_raw_handle()));
        }
    };
}

impl BigIntApi for VMHooksBackend {
    fn bi_new(&self, _value: i64) -> Self::BigIntHandle {
        todo!()
    }

    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64) {
        self.with_vm_hooks(|vh| vh.big_int_set_int64(destination.get_raw_handle(), value));
    }

    fn bi_unsigned_byte_length(&self, x: Self::BigIntHandle) -> usize {
        self.with_vm_hooks(|vh| vh.big_int_unsigned_byte_length(x.get_raw_handle()) as usize)
    }

    fn bi_get_unsigned_bytes(&self, _reference: Self::BigIntHandle) -> BoxedBytes {
        todo!()
    }

    fn bi_set_unsigned_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]) {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                vh.big_int_set_unsigned_bytes(destination, offset, length)
            })
        });
    }

    fn bi_signed_byte_length(&self, x: Self::BigIntHandle) -> usize {
        self.with_vm_hooks(|vh| vh.big_int_signed_byte_length(x.get_raw_handle()) as usize)
    }

    fn bi_get_signed_bytes(&self, _reference: Self::BigIntHandle) -> BoxedBytes {
        todo!()
    }

    fn bi_set_signed_bytes(&self, destination: Self::BigIntHandle, bytes: &[u8]) {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                vh.big_int_set_signed_bytes(destination, offset, length)
            })
        });
    }

    fn bi_to_i64(&self, _reference: Self::BigIntHandle) -> Option<i64> {
        todo!()
    }

    binary_op_method! {bi_add, big_int_add}
    binary_op_method! {bi_sub, big_int_sub}

    fn bi_sub_unsigned(
        &self,
        _dest: Self::BigIntHandle,
        _x: Self::BigIntHandle,
        _y: Self::BigIntHandle,
    ) {
        todo!()
    }

    binary_op_method! {bi_mul, big_int_mul}
    binary_op_method! {bi_t_div, big_int_tdiv}
    binary_op_method! {bi_t_mod, big_int_tmod}

    unary_op_method! {bi_abs, big_int_abs}
    unary_op_method! {bi_neg, big_int_neg}

    fn bi_sign(&self, x: Self::BigIntHandle) -> Sign {
        let sign_raw = self.with_vm_hooks(|vh| vh.big_int_sign(x.get_raw_handle()));
        match sign_raw.cmp(&0) {
            Ordering::Greater => Sign::Plus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Less => Sign::Minus,
        }
    }

    fn bi_cmp(&self, x: Self::BigIntHandle, y: Self::BigIntHandle) -> Ordering {
        let ordering_raw =
            self.with_vm_hooks(|vh| vh.big_int_cmp(x.get_raw_handle(), y.get_raw_handle()));
        ordering_raw.cmp(&0)
    }

    unary_op_method! {bi_sqrt, big_int_sqrt}
    binary_op_method! {bi_pow, big_int_pow}

    fn bi_log2(&self, x: Self::BigIntHandle) -> u32 {
        self.with_vm_hooks(|vh| vh.big_int_log2(x.get_raw_handle())) as u32
    }

    binary_op_method! {bi_and, big_int_and}
    binary_op_method! {bi_or, big_int_or}
    binary_op_method! {bi_xor, big_int_xor}

    fn bi_shr(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        self.with_vm_hooks(|vh| {
            vh.big_int_shr(dest.get_raw_handle(), x.get_raw_handle(), bits as i32)
        });
    }

    fn bi_shl(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        self.with_vm_hooks(|vh| {
            vh.big_int_shl(dest.get_raw_handle(), x.get_raw_handle(), bits as i32)
        });
    }

    fn bi_to_string(&self, _bi_handle: Self::BigIntHandle, _str_handle: Self::ManagedBufferHandle) {
        todo!()
    }
}
