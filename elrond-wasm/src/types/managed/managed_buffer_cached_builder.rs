use elrond_codec::{EncodeError, NestedEncodeOutput, TryStaticCast};

use crate::{api::ManagedTypeApi, types::StaticBufferRef};

use super::{BigInt, BigUint, ManagedBuffer, ManagedBufferSizeContext};

pub struct ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    managed_buffer: ManagedBuffer<M>,
    static_cache: Option<StaticBufferRef<M>>,
}

impl<M> ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    /// Creates instance as lazily as possible.
    /// If possible, the slice is loaded into the static buffer.
    /// If not, it is saved into the managed buffer so that the data is not lost.
    /// Use `flush_to_managed_buffer` after this to ensure that the managed buffer is populated.
    pub fn new_from_slice(slice: &[u8]) -> Self {
        let static_cache = StaticBufferRef::try_new(slice);
        if static_cache.is_some() {
            ManagedBufferCachedBuilder {
                managed_buffer: ManagedBuffer::new(),
                static_cache,
            }
        } else {
            ManagedBufferCachedBuilder {
                managed_buffer: slice.into(),
                static_cache: None,
            }
        }
    }

    pub fn into_managed_buffer(mut self) -> ManagedBuffer<M> {
        self.flush_to_managed_buffer();
        self.managed_buffer
    }

    fn flush_to_managed_buffer(&mut self) {
        let old_static_cache = core::mem::replace(&mut self.static_cache, None);
        if let Some(static_cache) = &old_static_cache {
            static_cache.with_buffer_contents(|bytes| {
                self.managed_buffer.append_bytes(bytes);
            });
        }
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
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

    pub fn append_managed_buffer(&mut self, item: &ManagedBuffer<M>) {
        let mut static_cache_mut = core::mem::replace(&mut self.static_cache, None);
        if let Some(static_cache) = &mut static_cache_mut {
            let success = static_cache.try_extend_from_copy_bytes(item.len(), |dest_slice| {
                let _ = item.load_slice(0, dest_slice);
            });
            if !success {
                self.flush_to_managed_buffer();
                self.managed_buffer.append(item);
            }
        } else {
            self.managed_buffer.append(item);
        }
        let _ = core::mem::replace(&mut self.static_cache, static_cache_mut);
    }

    #[inline]
    fn push_nested_managed_buffer(&mut self, item: &ManagedBuffer<M>) {
        let len_bytes = (item.len() as u32).to_be_bytes();
        self.append_bytes(&len_bytes[..]);
        self.append_managed_buffer(item);
    }
}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBufferCachedBuilder<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn push_specialized<T, C, F>(
        &mut self,
        context: C,
        value: &T,
        else_serialization: F,
    ) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<(), EncodeError>,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            if context.try_cast_ref::<ManagedBufferSizeContext>().is_some() {
                // managed buffers originating from fixed-length types don't need to serialize the length
                self.append_managed_buffer(managed_buffer);
            } else {
                self.push_nested_managed_buffer(managed_buffer);
            }
            Ok(())
        } else if let Some(big_uint) = value.try_cast_ref::<BigUint<M>>() {
            self.push_nested_managed_buffer(&big_uint.to_bytes_be_buffer());
            Ok(())
        } else if let Some(big_int) = value.try_cast_ref::<BigInt<M>>() {
            self.push_nested_managed_buffer(&big_int.to_signed_bytes_be_buffer());
            Ok(())
        } else {
            else_serialization(self)
        }
    }
}
