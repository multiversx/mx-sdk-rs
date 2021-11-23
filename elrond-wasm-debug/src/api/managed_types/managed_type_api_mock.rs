use elrond_wasm::api::{BigIntApi, Handle, ManagedTypeApi};
use num_bigint::{BigInt, Sign};

use crate::DebugApi;

impl ManagedTypeApi for DebugApi {
    fn instance() -> Self {
        DebugApi::new_from_static()
    }

    fn mb_to_big_int_unsigned(&self, buffer_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(bi)
    }

    fn mb_to_big_int_signed(&self, buffer_handle: Handle) -> Handle {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle);
        let bi = BigInt::from_signed_bytes_be(bytes.as_slice());
        managed_types.big_int_map.insert_new_handle(bi)
    }

    fn mb_from_big_int_unsigned(&self, big_int_handle: Handle) -> Handle {
        let bi_bytes = self.bi_get_unsigned_bytes(big_int_handle);
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert_new_handle(bi_bytes.into_vec())
    }

    fn mb_from_big_int_signed(&self, big_int_handle: Handle) -> Handle {
        let bi_bytes = self.bi_get_signed_bytes(big_int_handle);
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_buffer_map
            .insert_new_handle(bi_bytes.into_vec())
    }

    fn mb_overwrite_static_buffer(&self, buffer_handle: Handle) -> bool {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle).clone();
        managed_types
            .lockable_static_buffer
            .try_lock_with_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes.as_slice()))
    }

    fn append_mb_to_static_buffer(&self, buffer_handle: Handle) -> bool {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.managed_buffer_map.get(buffer_handle).clone();
        managed_types
            .lockable_static_buffer
            .try_extend_from_copy_bytes(bytes.len(), |dest| dest.copy_from_slice(bytes.as_slice()))
    }

    fn append_static_buffer_to_mb(&self, buffer_handle: Handle) {
        let mut managed_types = self.m_types_borrow_mut();
        let bytes = managed_types.lockable_static_buffer.as_slice().to_vec();
        managed_types
            .managed_buffer_map
            .get_mut(buffer_handle)
            .extend_from_slice(bytes.as_slice());
    }
}
