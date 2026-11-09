#![cfg_attr(not(feature = "managed-buffer-preload"), allow(unused))]

//! Managed buffer wrapper that copies its contents into the static buffer on first read,
//! so subsequent fixed-size reads are served from wasm memory instead of VM calls.
//!
//! Only active with the `managed-buffer-preload` feature.

use crate::{
    api::{ManagedBufferApiImpl, ManagedTypeApi, quick_signal_error},
    err_msg,
    types::{ManagedBuffer, ManagedType, StaticBufferRef},
};

pub(crate) struct PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub managed_buffer: ManagedBuffer<M>,
    pub buffer_len: usize,
    static_cache: Option<StaticBufferRef<M>>,
}

impl<M> PreloadedManagedBuffer<M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: ManagedBuffer<M>) -> Self {
        let buffer_len = managed_buffer.len();
        Self {
            managed_buffer,
            buffer_len,
            static_cache: None,
        }
    }

    fn try_load_static_cache_if_necessary(&mut self) {
        if self.static_cache.is_some() {
            return;
        }

        self.static_cache =
            StaticBufferRef::try_new_from_copy_bytes(self.buffer_len, |dest_slice| {
                self.managed_buffer.load_slice(0, dest_slice);
            });
    }

    pub fn load_slice(&mut self, starting_position: usize, dest_slice: &mut [u8]) {
        self.try_load_static_cache_if_necessary();
        if let Some(static_cache) = &self.static_cache {
            let result = static_cache.load_slice(starting_position, dest_slice);
            if result.is_err() {
                quick_signal_error::<M>(err_msg::BAD_MB_SLICE);
            }
        } else {
            self.managed_buffer
                .load_slice(starting_position, dest_slice);
        }
    }

    /// Creates a new managed buffer from a slice of the contents.
    ///
    /// When the static cache is loaded, the new buffer is created from the cached bytes.
    /// This matters for gas: copying a slice via the VM is charged for the length of the entire source buffer,
    /// whereas creating a buffer from bytes is only charged for the length of the slice.
    pub fn copy_slice(
        &mut self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        self.try_load_static_cache_if_necessary();
        if let Some(static_cache) = &self.static_cache {
            let end_position = starting_position.checked_add(slice_len)?;
            if end_position > self.buffer_len {
                return None;
            }

            // The handle must be allocated before accessing the static buffer:
            // on some APIs (e.g. StaticApi) both are guarded by the same lock.
            let result = unsafe { ManagedBuffer::<M>::new_uninit() };
            static_cache.with_buffer_contents(|bytes| {
                M::managed_type_impl()
                    .mb_overwrite(result.get_handle(), &bytes[starting_position..end_position]);
            });
            Some(result)
        } else {
            self.managed_buffer.copy_slice(starting_position, slice_len)
        }
    }
}
