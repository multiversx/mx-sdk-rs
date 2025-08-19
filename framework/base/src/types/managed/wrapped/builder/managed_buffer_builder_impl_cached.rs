use crate::{
    api::ManagedTypeApi,
    types::{ManagedBuffer, StaticBufferRef},
};

use super::ManagedBufferBuilderImpl;

/// A ManagedBuffer builder implementation that caches data to the static cache locally in the contract.
pub struct ManagedBufferBuilderImplCached<M>
where
    M: ManagedTypeApi,
{
    managed_buffer: ManagedBuffer<M>,
    static_cache: Option<StaticBufferRef<M>>,
}

impl<M> ManagedBufferBuilderImplCached<M>
where
    M: ManagedTypeApi,
{
    fn flush_to_managed_buffer(&mut self) {
        let old_static_cache = core::mem::take(&mut self.static_cache);
        if let Some(static_cache) = &old_static_cache {
            static_cache.with_buffer_contents(|bytes| {
                self.managed_buffer.append_bytes(bytes);
            });
        }
    }
}

impl<M> ManagedBufferBuilderImpl<M> for ManagedBufferBuilderImplCached<M>
where
    M: ManagedTypeApi,
{
    /// Creates instance as lazily as possible.
    /// If possible, the slice is loaded into the static buffer.
    /// If not, it is saved into the managed buffer so that the data is not lost.
    /// Use `flush_to_managed_buffer` after this to ensure that the managed buffer is populated.
    fn new_from_slice(slice: &[u8]) -> Self {
        let static_cache = StaticBufferRef::try_new(slice);
        if static_cache.is_some() {
            ManagedBufferBuilderImplCached {
                managed_buffer: ManagedBuffer::new(),
                static_cache,
            }
        } else {
            ManagedBufferBuilderImplCached {
                managed_buffer: slice.into(),
                static_cache: None,
            }
        }
    }

    fn into_managed_buffer(mut self) -> ManagedBuffer<M> {
        self.flush_to_managed_buffer();
        self.managed_buffer
    }

    fn append_bytes(&mut self, bytes: &[u8]) {
        if let Some(static_cache) = &mut self.static_cache {
            let success = static_cache.try_extend_from_slice(bytes);
            if !success {
                self.flush_to_managed_buffer();
                self.managed_buffer.append_bytes(bytes);
            }
        } else {
            self.managed_buffer.append_bytes(bytes);
        }
    }

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<M>) {
        if let Some(static_cache) = &mut self.static_cache {
            let success = static_cache.try_extend_from_copy_bytes(item.len(), |dest_slice| {
                item.load_slice(0, dest_slice);
            });
            if !success {
                self.flush_to_managed_buffer();
                self.managed_buffer.append(item);
            }
        } else {
            self.managed_buffer.append(item);
        }
    }
}
