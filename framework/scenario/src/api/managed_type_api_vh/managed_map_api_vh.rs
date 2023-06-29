use multiversx_sc::api::{use_raw_handle, HandleConstraints, ManagedMapApiImpl};

use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> ManagedMapApiImpl for VMHooksApi<VHB> {
    fn mm_new(&self) -> Self::ManagedMapHandle {
        let raw_handle = self.with_vm_hooks(|vh| vh.managed_map_new());
        use_raw_handle(raw_handle)
    }

    fn mm_get(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_3(&map_handle, &key_handle, &out_value_handle, |vh| {
            vh.managed_map_get(
                map_handle.get_raw_handle_unchecked(),
                key_handle.get_raw_handle_unchecked(),
                out_value_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mm_put(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_3(&map_handle, &key_handle, &value_handle, |vh| {
            vh.managed_map_put(
                map_handle.get_raw_handle_unchecked(),
                key_handle.get_raw_handle_unchecked(),
                value_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mm_remove(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
        out_value_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_3(&map_handle, &key_handle, &out_value_handle, |vh| {
            vh.managed_map_remove(
                map_handle.get_raw_handle_unchecked(),
                key_handle.get_raw_handle_unchecked(),
                out_value_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mm_contains(
        &self,
        map_handle: Self::ManagedMapHandle,
        key_handle: Self::ManagedBufferHandle,
    ) -> bool {
        i32_to_bool(self.with_vm_hooks_ctx_2(&map_handle, &key_handle, |vh| {
            vh.managed_map_contains(
                map_handle.get_raw_handle_unchecked(),
                key_handle.get_raw_handle_unchecked(),
            )
        }))
    }
}
