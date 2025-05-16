use crate::{
    host::vm_hooks::{VMHooksContext, VMHooksHandler},
    types::RawHandle,
};

impl<C: VMHooksContext> VMHooksHandler<C> {
    pub fn mm_new(&self) -> RawHandle {
        self.context.m_types_lock().mm_new()
    }

    pub fn mm_get(
        &self,
        map_handle: RawHandle,
        key_handle: RawHandle,
        out_value_handle: RawHandle,
    ) {
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .context
            .m_types_lock()
            .mm_values_get(map_handle, key.as_slice());
        self.context.m_types_lock().mb_set(out_value_handle, value);
    }

    pub fn mm_put(&self, map_handle: RawHandle, key_handle: RawHandle, value_handle: RawHandle) {
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self.context.m_types_lock().mb_get(value_handle).to_vec();
        self.context
            .m_types_lock()
            .mm_values_insert(map_handle, key, value);
    }

    pub fn mm_remove(
        &self,
        map_handle: RawHandle,
        key_handle: RawHandle,
        out_value_handle: RawHandle,
    ) {
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .context
            .m_types_lock()
            .mm_values_remove(map_handle, key.as_slice());
        self.context.m_types_lock().mb_set(out_value_handle, value);
    }

    pub fn mm_contains(&self, map_handle: RawHandle, key_handle: RawHandle) -> bool {
        let key = self.context.m_types_lock().mb_get(key_handle).to_vec();
        self.context
            .m_types_lock()
            .mm_contains(map_handle, key.as_slice())
    }
}
