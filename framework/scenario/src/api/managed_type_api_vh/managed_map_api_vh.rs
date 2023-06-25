use multiversx_sc::api::{use_raw_handle, ManagedMapApiImpl};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> ManagedMapApiImpl for VMHooksApi<VHB> {
    fn mm_new(&self) -> Self::ManagedBufferHandle {
        let raw_handle = self.with_vm_hooks(|vh| vh.managed_map_new());
        use_raw_handle(raw_handle)
    }

    fn mm_get(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_map_get(map_handle, key_handle, out_value_handle));
    }

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_map_put(map_handle, key_handle, out_value_handle));
    }

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_map_remove(map_handle, key_handle, out_value_handle));
    }

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        i32_to_bool(self.with_vm_hooks(|vh| vh.managed_map_contains(map_handle, key_handle)))
    }
}
