use core::cmp::Ordering;

use multiversx_sc::api::{use_raw_handle, BigFloatApiImpl, HandleConstraints, Sign};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

macro_rules! binary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigFloatHandle,
            x: Self::BigFloatHandle,
            y: Self::BigFloatHandle,
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

macro_rules! unary_op_wrapper {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle) {
            self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
                vh.$hook_name(
                    dest.get_raw_handle_unchecked(),
                    x.get_raw_handle_unchecked(),
                )
            });
        }
    };
}

macro_rules! unary_op_method_big_int_handle {
    ($method_name:ident, $hook_name:ident) => {
        fn $method_name(&self, dest: Self::BigIntHandle, x: Self::BigFloatHandle) {
            self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
                vh.$hook_name(
                    dest.get_raw_handle_unchecked(),
                    x.get_raw_handle_unchecked(),
                )
            });
        }
    };
}

impl<VHB: VMHooksApiBackend> BigFloatApiImpl for VMHooksApi<VHB> {
    fn bf_from_parts(
        &self,
        integral_part_value: i32,
        fractional_part_value: i32,
        exponent_value: i32,
    ) -> Self::BigFloatHandle {
        let raw_handle = self.with_vm_hooks(|vh| {
            vh.big_float_new_from_parts(integral_part_value, fractional_part_value, exponent_value)
        });
        use_raw_handle(raw_handle)
    }

    fn bf_from_frac(&self, numerator_value: i64, denominator_value: i64) -> Self::BigFloatHandle {
        let raw_handle =
            self.with_vm_hooks(|vh| vh.big_float_new_from_frac(numerator_value, denominator_value));
        use_raw_handle(raw_handle)
    }

    fn bf_from_sci(&self, significand_value: i64, exponent_value: i64) -> Self::BigFloatHandle {
        let raw_handle =
            self.with_vm_hooks(|vh| vh.big_float_new_from_sci(significand_value, exponent_value));
        use_raw_handle(raw_handle)
    }

    binary_op_wrapper! {bf_add, big_float_add}
    binary_op_wrapper! {bf_sub, big_float_sub}
    binary_op_wrapper! {bf_mul, big_float_mul}
    binary_op_wrapper! {bf_div, big_float_div}

    unary_op_wrapper! {bf_neg, big_float_neg}
    unary_op_wrapper! {bf_abs, big_float_abs}

    fn bf_cmp(&self, x: Self::BigFloatHandle, y: Self::BigFloatHandle) -> Ordering {
        let result = self.with_vm_hooks_ctx_2(&x, &y, |vh| {
            vh.big_float_cmp(x.get_raw_handle_unchecked(), y.get_raw_handle_unchecked())
        });
        result.cmp(&0)
    }

    fn bf_sign(&self, x: Self::BigFloatHandle) -> Sign {
        let result =
            self.with_vm_hooks_ctx_1(&x, |vh| vh.big_float_sign(x.get_raw_handle_unchecked()));
        match result.cmp(&0) {
            Ordering::Greater => Sign::Plus,
            Ordering::Equal => Sign::NoSign,
            Ordering::Less => Sign::Minus,
        }
    }

    unary_op_wrapper! {bf_clone, big_float_clone}
    unary_op_wrapper! {bf_sqrt, big_float_sqrt}

    fn bf_pow(&self, dest: Self::BigFloatHandle, x: Self::BigFloatHandle, exp: i32) {
        self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
            vh.big_float_pow(
                dest.get_raw_handle_unchecked(),
                x.get_raw_handle_unchecked(),
                exp,
            )
        });
    }

    unary_op_method_big_int_handle! {bf_floor , big_float_floor}
    unary_op_method_big_int_handle! {bf_ceil ,  big_float_ceil}
    unary_op_method_big_int_handle! {bf_trunc , big_float_truncate}

    fn bf_is_bi(&self, x: Self::BigFloatHandle) -> bool {
        i32_to_bool(
            self.with_vm_hooks_ctx_1(&x, |vh| vh.big_float_is_int(x.get_raw_handle_unchecked())),
        )
    }

    fn bf_set_i64(&self, dest: Self::BigFloatHandle, value: i64) {
        self.with_vm_hooks_ctx_1(&dest, |vh| {
            vh.big_float_set_int64(dest.get_raw_handle_unchecked(), value)
        });
    }

    fn bf_set_bi(&self, dest: Self::BigFloatHandle, x: Self::BigIntHandle) {
        self.with_vm_hooks_ctx_2(&dest, &x, |vh| {
            vh.big_float_set_big_int(
                dest.get_raw_handle_unchecked(),
                x.get_raw_handle_unchecked(),
            )
        });
    }

    fn bf_get_const_e(&self, dest: Self::BigFloatHandle) {
        self.with_vm_hooks_ctx_1(&dest, |vh| {
            vh.big_float_get_const_e(dest.get_raw_handle_unchecked())
        });
    }

    fn bf_get_const_pi(&self, dest: Self::BigFloatHandle) {
        self.with_vm_hooks_ctx_1(&dest, |vh| {
            vh.big_float_get_const_pi(dest.get_raw_handle_unchecked())
        });
    }
}
