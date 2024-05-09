use core::marker::PhantomData;

use crate::codec::{EncodeError, EncodeErrorHandler, NestedEncodeOutput, TryStaticCast};

use crate::{
    api::ManagedTypeApi,
    formatter::{
        hex_util::{byte_to_binary_digits, byte_to_hex_digits},
        FormatBuffer, FormatByteReceiver, SCBinary, SCCodec, SCDisplay, SCLowerHex,
    },
    types::ManagedBuffer,
};

use super::{ManagedBufferBuilderImpl, ManagedBufferImplDefault};

const HEX_CONVERSION_BUFFER_LEN: usize = 32;
const BIN_CONVERSION_BUFFER_LEN: usize = 32;

pub struct ManagedBufferBuilder<M, Impl = ManagedBufferImplDefault<M>>
where
    M: ManagedTypeApi,
    Impl: ManagedBufferBuilderImpl<M>,
{
    _phantom: PhantomData<M>,
    implementation: Impl,
}

impl<M, Impl> ManagedBufferBuilder<M, Impl>
where
    M: ManagedTypeApi,
    Impl: ManagedBufferBuilderImpl<M>,
{
    /// Creates instance as lazily as possible.
    /// If possible, the slice is loaded into the static buffer.
    /// If not, it is saved into the managed buffer so that the data is not lost.
    /// Use `flush_to_managed_buffer` after this to ensure that the managed buffer is populated.
    pub fn new_from_slice(slice: &[u8]) -> Self {
        ManagedBufferBuilder {
            _phantom: PhantomData,
            implementation: Impl::new_from_slice(slice),
        }
    }
}

impl<M> Default for ManagedBufferBuilder<M, ManagedBufferImplDefault<M>>
where
    M: ManagedTypeApi,
{
    #[inline]
    fn default() -> Self {
        Self::new_from_slice(&[])
    }
}

impl<M, Impl> ManagedBufferBuilder<M, Impl>
where
    M: ManagedTypeApi,
    Impl: ManagedBufferBuilderImpl<M>,
{
    pub fn into_managed_buffer(self) -> ManagedBuffer<M> {
        self.implementation.into_managed_buffer()
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.implementation.append_bytes(bytes);
    }

    pub fn append_managed_buffer(&mut self, item: &ManagedBuffer<M>) {
        self.implementation.append_managed_buffer(item);
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

impl<M, Impl> NestedEncodeOutput for ManagedBufferBuilder<M, Impl>
where
    M: ManagedTypeApi,
    Impl: ManagedBufferBuilderImpl<M>,
{
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<M>>()
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

impl<M, Impl> FormatByteReceiver for ManagedBufferBuilder<M, Impl>
where
    M: ManagedTypeApi,
    Impl: ManagedBufferBuilderImpl<M>,
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

impl<M> FormatBuffer for ManagedBufferBuilder<M, ManagedBufferImplDefault<M>>
where
    M: ManagedTypeApi,
{
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
