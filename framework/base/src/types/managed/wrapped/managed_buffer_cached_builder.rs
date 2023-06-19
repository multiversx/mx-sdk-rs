use crate::codec::{EncodeError, EncodeErrorHandler, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    formatter::{
        hex_util::{byte_to_binary_digits, byte_to_hex_digits},
        FormatBuffer, FormatByteReceiver, SCBinary, SCCodec, SCDisplay, SCLowerHex,
    },
    types::{BigInt, BigUint, ManagedBuffer, StaticBufferRef},
};

const HEX_CONVERSION_BUFFER_LEN: usize = 32;
const BIN_CONVERSION_BUFFER_LEN: usize = 32;

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
}

impl<M> Default for ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn default() -> Self {
        Self::new_from_slice(&[])
    }
}

impl<M> ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    pub fn into_managed_buffer(mut self) -> ManagedBuffer<M> {
        self.flush_to_managed_buffer();
        self.managed_buffer
    }

    fn flush_to_managed_buffer(&mut self) {
        let old_static_cache = core::mem::take(&mut self.static_cache);
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
        if let Some(static_cache) = &mut self.static_cache {
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
    }

    /// Converts the input to hex and adds it to the current buffer.
    ///
    /// TODO: consider making the hex conversion part of the VM
    pub fn append_managed_buffer_hex(&mut self, item: &ManagedBuffer<M>) {
        item.for_each_batch::<HEX_CONVERSION_BUFFER_LEN, _>(|batch| {
            let mut hex_bytes_buffer = [0u8; HEX_CONVERSION_BUFFER_LEN * 2];
            for (i, &byte) in batch.iter().enumerate() {
                let digits = byte_to_hex_digits(byte);
                hex_bytes_buffer[i * 2] = digits[0];
                hex_bytes_buffer[i * 2 + 1] = digits[1];
            }
            let hex_slice = &hex_bytes_buffer[0..(batch.len() * 2)];
            self.append_bytes(hex_slice);
        });
    }

    /// Converts the input to binary ASCII and adds it to the current buffer.
    pub fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<M>) {
        item.for_each_batch::<BIN_CONVERSION_BUFFER_LEN, _>(|batch| {
            for &byte in batch {
                let ascii_bin_digit = byte_to_binary_digits(byte);
                self.append_bytes(&ascii_bin_digit[..]);
            }
        });
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

impl<M> FormatByteReceiver for ManagedBufferCachedBuilder<M>
where
    M: ManagedTypeApi,
{
    type Api = M;

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes)
    }

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<M>) {
        self.append_managed_buffer(item)
    }

    fn append_managed_buffer_lower_hex(&mut self, item: &ManagedBuffer<M>) {
        self.append_managed_buffer_hex(item)
    }

    fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<M>) {
        self.append_managed_buffer_binary(item)
    }
}

impl<M: ManagedTypeApi> FormatBuffer for ManagedBufferCachedBuilder<M> {
    fn append_ascii(&mut self, ascii: &[u8]) {
        self.append_bytes(ascii)
    }

    fn append_display<T: SCDisplay>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_lower_hex<T: SCLowerHex>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_binary<T: SCBinary>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_codec<T: SCCodec>(&mut self, item: &T) {
        item.fmt(self);
    }
}
