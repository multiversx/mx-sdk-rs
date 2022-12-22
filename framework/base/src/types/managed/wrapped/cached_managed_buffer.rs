use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::{ManagedBuffer, StaticBufferRef},
};

fn load_static_cache<M>(managed_buffer: &ManagedBuffer<M>) -> StaticBufferRef<M>
where
    M: ManagedTypeApi,
{
    StaticBufferRef::try_new_from_copy_bytes(managed_buffer.len(), |dest_slice| {
        let _ = managed_buffer.load_slice(0, dest_slice);
    })
    .unwrap_or_else(|| {
        M::error_api_impl().signal_error(b"static cache too small or already in use")
    })
}

pub fn with_buffer_contents<M, R, F>(managed_buffer: &ManagedBuffer<M>, f: F) -> R
where
    M: ManagedTypeApi,
    F: FnOnce(&[u8]) -> R,
{
    let static_cache = load_static_cache(managed_buffer);
    static_cache.with_buffer_contents(f)
}

pub fn with_buffer_contents_mut<M, F>(managed_buffer: &mut ManagedBuffer<M>, f: F)
where
    M: ManagedTypeApi,
    F: FnOnce(&mut [u8]) -> &[u8],
{
    let static_cache = load_static_cache(managed_buffer);
    static_cache.with_buffer_contents_mut(|buffer| {
        let result = f(buffer);
        managed_buffer.overwrite(result);
    });
}
