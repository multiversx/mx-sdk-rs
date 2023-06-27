use crate::{tx_mock::ManagedMapImpl, DebugApi};
use multiversx_sc::api::{ManagedMapApiImpl, HandleConstraints};

impl ManagedMapApiImpl for DebugApi {
    fn mm_new(&self) -> Self::ManagedMapHandle {
        let mut managed_types = self.m_types_borrow_mut();
        managed_types
            .managed_map_map
            .insert_new_handle(ManagedMapImpl::new())
    }

    fn mm_get(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        let key = self.mb_get(key_handle);
        let value = self
            .m_types_borrow()
            .mm_values_get(map_handle.get_raw_handle_unchecked(), key.as_slice());
        self.m_types_borrow_mut()
            .mb_set(out_value_handle.get_raw_handle_unchecked(), value);
    }

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        let key = self
            .m_types_borrow()
            .mb_get(key_handle.get_raw_handle_unchecked())
            .to_vec();
        let value = self
            .m_types_borrow()
            .mb_get(value_handle.get_raw_handle_unchecked())
            .to_vec();
        self.m_types_borrow_mut().mm_values_insert(
            map_handle.get_raw_handle_unchecked(),
            key,
            value,
        );
    }

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        let key = self
            .m_types_borrow()
            .mb_get(key_handle.get_raw_handle_unchecked())
            .to_vec();
        let value = self
            .m_types_borrow_mut()
            .mm_values_remove(map_handle.get_raw_handle_unchecked(), key.as_slice());
        self.mb_set(out_value_handle, value);
    }

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        let key = self
            .m_types_borrow()
            .mb_get(key_handle.get_raw_handle_unchecked())
            .to_vec();
        self.m_types_borrow_mut()
            .mm_contains(map_handle.get_raw_handle_unchecked(), key.as_slice())
    }
}
