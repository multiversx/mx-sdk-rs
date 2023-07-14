use crate::api::{i32_to_bool, VMHooksApi, VMHooksApiBackend};
use multiversx_chain_vm::{executor::MemPtr, mem_conv};
use multiversx_sc::{
    api::{use_raw_handle, HandleConstraints, InvalidSliceError, ManagedBufferApiImpl},
    types::BoxedBytes,
};

impl<VHB: VMHooksApiBackend> ManagedBufferApiImpl for VMHooksApi<VHB> {
    fn mb_new_empty(&self) -> Self::ManagedBufferHandle {
        let raw_handle = self.with_vm_hooks(|vh| vh.mbuffer_new());
        use_raw_handle(raw_handle)
    }

    fn mb_new_from_bytes(&self, bytes: &[u8]) -> Self::ManagedBufferHandle {
        let raw_handle = self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                vh.mbuffer_new_from_bytes(offset, length)
            })
        });
        use_raw_handle(raw_handle)
    }

    fn mb_len(&self, handle: Self::ManagedBufferHandle) -> usize {
        self.with_vm_hooks_ctx_1(&handle, |vh| {
            vh.mbuffer_get_length(handle.get_raw_handle_unchecked()) as usize
        })
    }

    fn mb_to_boxed_bytes(&self, handle: Self::ManagedBufferHandle) -> BoxedBytes {
        self.with_vm_hooks_ctx_1(&handle, |vh| {
            let len = vh.mbuffer_get_length(handle.get_raw_handle_unchecked()) as usize;
            unsafe {
                let mut res = BoxedBytes::allocate(len);
                if len > 0 {
                    let _ = vh.mbuffer_get_bytes(
                        handle.get_raw_handle_unchecked(),
                        res.as_mut_ptr() as MemPtr,
                    );
                }
                res
            }
        })
    }

    fn mb_load_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        let err = self.with_vm_hooks_ctx_1(&source_handle, |vh| {
            mem_conv::with_mem_ptr_mut(dest_slice, |offset, length| {
                vh.mbuffer_get_byte_slice(
                    source_handle.get_raw_handle_unchecked(),
                    starting_position as i32,
                    length as i32,
                    offset,
                )
            })
        });
        if err == 0 {
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_copy_slice(
        &self,
        source_handle: Self::ManagedBufferHandle,
        starting_pos: usize,
        slice_len: usize,
        dest_handle: Self::ManagedBufferHandle,
    ) -> Result<(), InvalidSliceError> {
        let err = self.with_vm_hooks_ctx_2(&source_handle, &dest_handle, |vh| {
            vh.mbuffer_copy_byte_slice(
                source_handle.get_raw_handle_unchecked(),
                starting_pos as i32,
                slice_len as i32,
                dest_handle.get_raw_handle_unchecked(),
            )
        });
        if err == 0 {
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_overwrite(&self, handle: Self::ManagedBufferHandle, value: &[u8]) {
        self.with_vm_hooks_ctx_1(&handle, |vh| {
            mem_conv::with_mem_ptr(value, |offset, length| {
                vh.mbuffer_set_bytes(handle.get_raw_handle_unchecked(), offset, length);
            })
        });
    }

    fn mb_set_slice(
        &self,
        dest_handle: Self::ManagedBufferHandle,
        starting_position: usize,
        source_slice: &[u8],
    ) -> Result<(), InvalidSliceError> {
        let err = self.with_vm_hooks_ctx_1(&dest_handle, |vh| {
            mem_conv::with_mem_ptr(source_slice, |offset, length| {
                vh.mbuffer_set_byte_slice(
                    dest_handle.get_raw_handle_unchecked(),
                    starting_position as i32,
                    length,
                    offset,
                )
            })
        });
        if err == 0 {
            Ok(())
        } else {
            Err(InvalidSliceError)
        }
    }

    fn mb_set_random(&self, dest_handle: Self::ManagedBufferHandle, length: usize) {
        self.with_vm_hooks_ctx_1(&dest_handle, |vh| {
            vh.mbuffer_set_random(dest_handle.get_raw_handle_unchecked(), length as i32)
        });
    }

    fn mb_append(
        &self,
        accumulator_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&accumulator_handle, &data_handle, |vh| {
            vh.mbuffer_append(
                accumulator_handle.get_raw_handle_unchecked(),
                data_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_append_bytes(&self, accumulator_handle: Self::ManagedBufferHandle, bytes: &[u8]) {
        self.with_vm_hooks_ctx_1(&accumulator_handle, |vh| {
            mem_conv::with_mem_ptr(bytes, |offset, length| {
                let _ = vh.mbuffer_append_bytes(
                    accumulator_handle.get_raw_handle_unchecked(),
                    offset,
                    length,
                );
            })
        });
    }

    fn mb_eq(
        &self,
        handle1: Self::ManagedBufferHandle,
        handle2: Self::ManagedBufferHandle,
    ) -> bool {
        i32_to_bool(self.with_vm_hooks_ctx_2(&handle1, &handle2, |vh| {
            vh.mbuffer_eq(
                handle1.get_raw_handle_unchecked(),
                handle2.get_raw_handle_unchecked(),
            )
        }))
    }

    fn mb_to_hex(
        &self,
        source_handle: Self::ManagedBufferHandle,
        dest_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&source_handle, &dest_handle, |vh| {
            vh.managed_buffer_to_hex(
                source_handle.get_raw_handle_unchecked(),
                dest_handle.get_raw_handle_unchecked(),
            )
        })
    }
}
