use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{ManagedBuffer, StaticBufferRef},
};

pub(crate) struct CachedManagedBuffer<'a, M>
where
    M: ManagedTypeApi,
{
    pub managed_buffer: &'a mut ManagedBuffer<M>,
}

impl<'a, M> CachedManagedBuffer<'a, M>
where
    M: ManagedTypeApi,
{
    pub fn new(managed_buffer: &'a mut ManagedBuffer<M>) -> Self {
        Self { managed_buffer }
    }

    fn load_static_cache(&mut self) -> StaticBufferRef<M> {
        StaticBufferRef::try_new_from_copy_bytes(self.managed_buffer.len(), |dest_slice| {
            let _ = self.managed_buffer.load_slice(0, dest_slice);
        })
        .unwrap_or_else(|| M::error_api_impl().signal_error(b"Static cache is in use"))
    }

    pub fn with_buffer_contents_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut [u8]),
    {
        let static_cache = self.load_static_cache();
        static_cache.with_buffer_contents_mut(f);
        static_cache.with_buffer_contents(|buffer| self.managed_buffer.overwrite(buffer))
    }
}
