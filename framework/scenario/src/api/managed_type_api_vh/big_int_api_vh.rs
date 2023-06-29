use core::cmp::Ordering;

use multiversx_sc::api::{use_raw_handle, BigIntApiImpl, HandleConstraints, Sign};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

macro_rules! binary_op_method {
    ($api_method_name:ident, $hook_name:ident) => {
        fn $api_method_name(
            &self,
            dest: Self::BigIntHandle,
            x: Self::BigIntHandle,
            y: Self::BigIntHandle,
        ) {
            self.with_vm_hooks_ctx_3(&dest, &x, &y, |vh| {
                vh.$hook_name(
                    dest.get_raw_handle_unchecked(),
                    x.get_raw_handle_unchecked(),
                    y.get_raw_handle_unchecked(),
                )
            });
        }
    };
}

macro_rules! unary_op_method {
    ($api_method_name:ident, $hook_name:ident) => {
        fn $api_method_name(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle) {
            self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
                vh.$hook_name(
                    dest.get_raw_handle_unchecked(),
                    x.get_raw_handle_unchecked(),
                )
            });
        }
    };
}

impl<VHB: VMHooksApiBackend> BigIntApiImpl for VMHooksApi<VHB> {
    fn bi_new(&self, value: i64) -> Self::BigIntHandle {
        let handle = self.with_vm_hooks(|vh| vh.big_int_new(value));
        use_raw_handle(handle)
    }

    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64) {
        self.with_vm_hooks_ctx_1(&destination, |vh| {
            vh.big_int_set_int64(destination.get_raw_handle_unchecked(), value)
        });
    }

    fn bi_to_i64(&self, reference: Self::BigIntHandle) -> Option<i64> {
        self.with_vm_hooks_ctx_1(&reference, |vh| {
            let is_i64_result = vh.big_int_is_int64(reference.get_raw_handle_unchecked());
            if i32_to_bool(is_i64_result) {
                Some(vh.big_int_get_int64(reference.get_raw_handle_unchecked()))
            } else {
                None
            }
        })
    }

    binary_op_method! {bi_add, big_int_add}
    binary_op_method! {bi_sub, big_int_sub}
    binary_op_method! {bi_mul, big_int_mul}
    binary_op_method! {bi_t_div, big_int_tdiv}
    binary_op_method! {bi_t_mod, big_int_tmod}

    unary_op_method! {bi_abs, big_int_abs}
    unary_op_method! {bi_neg, big_int_neg}

    fn bi_sign(&self, x: Self::BigIntHandle) -> Sign {
        let sign_raw =
            self.with_vm_hooks_ctx_1(&x, |vh| vh.big_int_sign(x.get_raw_handle_unchecked()));
        match sign_raw.cmp(&0) {
            Ordering::Greater => Sign::Plus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Less => Sign::Minus,
        }
    }

    fn bi_cmp(&self, x: Self::BigIntHandle, y: Self::BigIntHandle) -> Ordering {
        let ordering_raw = self.with_vm_hooks_ctx_2(&x, &y, |vh| {
            vh.big_int_cmp(x.get_raw_handle_unchecked(), y.get_raw_handle_unchecked())
        });
        ordering_raw.cmp(&0)
    }

    unary_op_method! {bi_sqrt, big_int_sqrt}
    binary_op_method! {bi_pow, big_int_pow}

    fn bi_log2(&self, x: Self::BigIntHandle) -> u32 {
        self.with_vm_hooks_ctx_1(&x, |vh| vh.big_int_log2(x.get_raw_handle_unchecked())) as u32
    }

    binary_op_method! {bi_and, big_int_and}
    binary_op_method! {bi_or, big_int_or}
    binary_op_method! {bi_xor, big_int_xor}

    fn bi_shr(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
            vh.big_int_shr(
                dest.get_raw_handle_unchecked(),
                x.get_raw_handle_unchecked(),
                bits as i32,
            )
        });
    }

    fn bi_shl(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
            vh.big_int_shl(
                dest.get_raw_handle_unchecked(),
                x.get_raw_handle_unchecked(),
                bits as i32,
            )
        });
    }

    fn bi_to_string(&self, bi_handle: Self::BigIntHandle, str_handle: Self::ManagedBufferHandle) {
        self.with_vm_hooks_ctx_2(&bi_handle, &str_handle, |vh| {
            vh.big_int_to_string(
                bi_handle.get_raw_handle_unchecked(),
                str_handle.get_raw_handle_unchecked(),
            )
        });
    }
}
