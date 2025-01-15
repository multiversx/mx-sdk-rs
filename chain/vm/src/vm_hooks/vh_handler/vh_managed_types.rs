mod vh_big_float;
mod vh_big_int;
mod vh_managed_buffer;
mod vh_managed_map;

pub use vh_big_float::VMHooksBigFloat;
pub use vh_big_int::VMHooksBigInt;
pub use vh_managed_buffer::VMHooksManagedBuffer;
pub use vh_managed_map::VMHooksManagedMap;

use std::fmt::Debug;

use crate::{
    tx_mock::{big_int_signed_bytes, big_int_to_i64, big_uint_to_u64, big_uint_unsigned_bytes},
    types::RawHandle,
    vm_err_msg,
};

use super::VMHooksError;

/// Provides VM hook implementations for methods that deal with more than one type of managed type.
///
/// It is also the trait that unifies all managed type functionality.
pub trait VMHooksManagedTypes:
    VMHooksBigInt + VMHooksManagedBuffer + VMHooksManagedMap + VMHooksBigFloat + VMHooksError + Debug
{
    fn mb_to_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_unsigned_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_to_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        self.m_types_lock()
            .bi_set_signed_bytes(bi_handle, bytes.as_slice());
    }

    fn mb_from_big_int_unsigned(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_unsigned_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn mb_from_big_int_signed(&self, buffer_handle: RawHandle, bi_handle: RawHandle) {
        let bi_bytes = self.m_types_lock().bi_get_signed_bytes(bi_handle);
        self.m_types_lock().mb_set(buffer_handle, bi_bytes);
    }

    fn mb_to_small_int_unsigned(&self, buffer_handle: RawHandle) -> u64 {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        let bu = num_bigint::BigUint::from_bytes_be(&bytes);
        big_uint_to_u64(&bu).unwrap_or_else(|| self.vm_error(vm_err_msg::ERROR_BYTES_EXCEED_UINT64))
    }

    fn mb_to_small_int_signed(&self, buffer_handle: RawHandle) -> i64 {
        let bytes = self.m_types_lock().mb_to_bytes(buffer_handle);
        let bi = num_bigint::BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
        big_int_to_i64(&bi).unwrap_or_else(|| self.vm_error(vm_err_msg::ERROR_BYTES_EXCEED_INT64))
    }

    fn mb_from_small_int_unsigned(&self, buffer_handle: RawHandle, value: u64) {
        let bu = num_bigint::BigUint::from(value);
        let bytes = big_uint_unsigned_bytes(&bu);
        self.m_types_lock().mb_set(buffer_handle, bytes);
    }

    fn mb_from_small_int_signed(&self, buffer_handle: RawHandle, value: i64) {
        let bi = num_bigint::BigInt::from(value);
        let bytes = big_int_signed_bytes(&bi);
        self.m_types_lock().mb_set(buffer_handle, bytes);
    }

    fn bi_to_string(&self, bi_handle: RawHandle, str_handle: RawHandle) {
        let bi = self.m_types_lock().bi_get(bi_handle);
        let s = bi.to_string();
        self.m_types_lock().mb_set(str_handle, s.into_bytes());
    }

    fn mb_set_random(&self, dest_handle: RawHandle, length: usize) {
        let bytes = self.random_next_bytes(length);
        self.mb_set(dest_handle, bytes.as_slice());
    }
}
