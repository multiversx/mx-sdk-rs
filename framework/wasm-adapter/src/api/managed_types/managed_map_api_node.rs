use multiversx_sc::api::ManagedMapApiImpl;

#[allow(dead_code)]
extern "C" {
    fn managedMapNew() -> i32;
    fn managedMapPut(map_handle: i32, key_handle: i32, value_handle: i32) -> i32;
    fn managedMapGet(map_handle: i32, key_handle: i32, out_value_handle: i32) -> i32;
    fn managedMapRemove(map_handle: i32, key_handle: i32, out_value_handle: i32) -> i32;
    fn managedMapContains(map_handle: i32, key_handle: i32) -> i32;
}

impl ManagedMapApiImpl for crate::api::VmApiImpl {
    fn mm_new(&self) -> Self::ManagedBufferHandle {
        unsafe { managedMapNew() }
    }

    fn mm_get(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedMapGet(map_handle, key_handle, out_value_handle);
        }
    }

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedMapPut(map_handle, key_handle, out_value_handle);
        }
    }

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedMapRemove(map_handle, key_handle, out_value_handle);
        }
    }

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        unsafe { managedMapContains(map_handle, key_handle) > 0 }
    }
}
