use crate::{tx_mock::ManagedMapImpl, DebugApi};
use multiversx_sc::api::{HandleTypeInfo, ManagedMapApiImpl};

impl DebugApi {
    fn mm_values_insert(
        &self,
        map_handle: <Self as HandleTypeInfo>::ManagedMapHandle,
        key: Vec<u8>,
        value: Vec<u8>,
    ) {
        let mut managed_types = self.m_types_borrow_mut();
        let mmap = managed_types
            .managed_map_map
            .get_mut(map_handle.get_raw_handle_unchecked());
        mmap.insert(key, value);
    }

    fn mm_values_get(
        &self,
        map_handle: <Self as HandleTypeInfo>::ManagedMapHandle,
        key: &[u8],
    ) -> Vec<u8> {
        let managed_types = self.m_types_borrow();
        let mmap = managed_types
            .managed_map_map
            .get(map_handle.get_raw_handle_unchecked());
        mmap.get(key).cloned().unwrap_or_default()
    }

    fn mm_values_remove(
        &self,
        map_handle: <Self as HandleTypeInfo>::ManagedMapHandle,
        key: &[u8],
    ) -> Vec<u8> {
        let mut managed_types = self.m_types_borrow_mut();
        let mmap = managed_types
            .managed_map_map
            .get_mut(map_handle.get_raw_handle_unchecked());
        mmap.remove(key).unwrap_or_default()
    }
}

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
        let value = self.mm_values_get(map_handle, key.as_slice());
        self.mb_set(out_value_handle, value);
    }

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        let key = self.mb_get(key_handle);
        let value = self.mb_get(value_handle);
        self.mm_values_insert(map_handle, key, value);
    }

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        let key = self.mb_get(key_handle);
        let value = self.mm_values_remove(map_handle, key.as_slice());
        self.mb_set(out_value_handle, value);
    }

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        let managed_types = self.m_types_borrow();
        let mmap = managed_types
            .managed_map_map
            .get(map_handle.get_raw_handle_unchecked());
        let key = self.mb_get(key_handle);
        if let Some(value) = mmap.get(&key) {
            !value.is_empty()
        } else {
            false
        }
    }
}
