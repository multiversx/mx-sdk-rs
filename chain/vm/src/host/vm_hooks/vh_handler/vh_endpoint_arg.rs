use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;

use crate::{host::vm_hooks::VMHooksHandlerSource, vm_err_msg::ERROR_NO_CALLBACK_CLOSURE};

use crate::types::RawHandle;

use super::VMHooksManagedTypes;

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
pub trait VMHooksEndpointArgument: VMHooksHandlerSource + VMHooksManagedTypes {
    fn get_num_arguments(&mut self) -> i32 {
        self.input_ref().args.len() as i32
    }

    fn get_argument_len(&mut self, arg_index: i32) -> usize {
        let arg = self.input_ref().get_argument_vec_u8(arg_index);
        arg.len()
    }

    fn load_argument_managed_buffer(&mut self, arg_index: i32, dest: RawHandle) {
        let arg_bytes = self.input_ref().get_argument_vec_u8(arg_index);
        self.m_types_lock().mb_set(dest, arg_bytes);
    }

    fn load_argument_big_int_unsigned(&mut self, arg_index: i32, dest: RawHandle) {
        let arg_bytes = self.input_ref().get_argument_vec_u8(arg_index);
        self.m_types_lock().bi_set_unsigned_bytes(dest, &arg_bytes);
    }

    fn load_argument_big_int_signed(&mut self, arg_index: i32, dest: RawHandle) {
        let arg_bytes = self.input_ref().get_argument_vec_u8(arg_index);
        self.m_types_lock().bi_set_signed_bytes(dest, &arg_bytes);
    }

    fn get_argument_i64(&mut self, arg_index: i32) -> i64 {
        // specific implementation provided, in order to simulate the VM error (status 10 instead of 4)
        let bytes = self.input_ref().get_argument_vec_u8(arg_index);
        let bi = BigInt::from_signed_bytes_be(&bytes);
        if let Some(v) = bi.to_i64() {
            v
        } else {
            self.vm_error("argument out of range");
            0
        }
    }

    fn get_argument_u64(&mut self, arg_index: i32) -> u64 {
        // specific implementation provided, in order to simulate the VM error (status 10 instead of 4)
        let bytes = self.input_ref().get_argument_vec_u8(arg_index);
        let bu = BigUint::from_bytes_be(&bytes);
        if let Some(v) = bu.to_u64() {
            v
        } else {
            self.vm_error("argument out of range");
            0
        }
    }

    fn load_callback_closure_buffer(&mut self, dest: RawHandle) {
        if let Some(closure_data) = &self.input_ref().promise_callback_closure_data {
            self.m_types_lock().mb_set(dest, closure_data.clone());
        } else {
            self.vm_error(ERROR_NO_CALLBACK_CLOSURE);
        }
    }
}
