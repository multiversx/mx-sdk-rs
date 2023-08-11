use crate::{types::RawHandle, vm_hooks::VMHooksHandlerSource};

pub trait VMHooksManagedMap: VMHooksHandlerSource {
    fn mm_new(&self) -> RawHandle {
        self.m_types_lock().mm_new()
    }

    fn mm_get(&self, map_handle: RawHandle, key_handle: RawHandle, out_value_handle: RawHandle) {
        let key = self.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .m_types_lock()
            .mm_values_get(map_handle, key.as_slice());
        self.m_types_lock().mb_set(out_value_handle, value);
    }

    fn mm_put(&self, map_handle: RawHandle, key_handle: RawHandle, value_handle: RawHandle) {
        let key = self.m_types_lock().mb_get(key_handle).to_vec();
        let value = self.m_types_lock().mb_get(value_handle).to_vec();
        self.m_types_lock().mm_values_insert(map_handle, key, value);
    }

    fn mm_remove(&self, map_handle: RawHandle, key_handle: RawHandle, out_value_handle: RawHandle) {
        let key = self.m_types_lock().mb_get(key_handle).to_vec();
        let value = self
            .m_types_lock()
            .mm_values_remove(map_handle, key.as_slice());
        self.m_types_lock().mb_set(out_value_handle, value);
    }

    fn mm_contains(&self, map_handle: RawHandle, key_handle: RawHandle) -> bool {
        let key = self.m_types_lock().mb_get(key_handle).to_vec();
        self.m_types_lock().mm_contains(map_handle, key.as_slice())
    }
}
