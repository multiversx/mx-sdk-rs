use elrond_codec::{EncodeError, EncodeErrorHandler, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    contract_base::ManagedSerializer,
    formatter::FormatReceiver,
    types::{BigInt, BigUint, ManagedBuffer, StaticBufferRef},
};

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
}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBufferCachedBuilder<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>() || T::type_eq::<BigUint<M>>() || T::type_eq::<BigInt<M>>()
    }

    #[inline]
    fn push_specialized<T, C, H>(
        &mut self,
        _context: C,
        value: &T,
        h: H,
    ) -> Result<(), H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: EncodeErrorHandler,
    {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            self.append_managed_buffer(managed_buffer);
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }
}

impl<M> FormatReceiver for ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    fn push_static_ascii(&mut self, arg: &'static [u8]) {
        self.append_bytes(arg);
    }

    fn push_bytes(&mut self, item: &mut ManagedFormatter<M>) {
        let mb = ManagedSerializer::<M>::new().top_encode_to_managed_buffer(item); // std::fmt::Formatter custom type that builds a ManagedBuffer
        self.append_managed_buffer(&mb);
    }

    fn push_lower_hex(&mut self, item: &mut ManagedFormatter<M>) {
        let mb = ManagedSerializer::<M>::new().top_encode_to_managed_buffer(item); // lower hex
        crate::hex_util::add_arg_as_hex_to_buffer(self, &mb);
    }
}

pub struct ManagedFormatter<M: ManagedTypeApi>(ManagedBuffer<M>);

impl<M> ManagedFormatter<M>
where
    M: ManagedTypeApi,
{
    fn new() -> Self {
        Self(ManagedBuffer::new())
    }

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.0.append_bytes(bytes);
    }

    fn into_buffer(self) -> ManagedBuffer<M> {
        self.0
    }
}
