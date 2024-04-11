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

pub struct ManagedBufferBuilder<'a, M, Impl = ManagedBufferImplDefault<'a, M>>
where
    M: ManagedTypeApi<'a>,
    Impl: ManagedBufferBuilderImpl<'a, M>,
{
    _phantom: PhantomData<M>,
    implementation: Impl,
}

impl<'a, M, Impl> ManagedBufferBuilder<'a, M, Impl>
where
    M: ManagedTypeApi<'a>,
    Impl: ManagedBufferBuilderImpl<'a, M>,
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

impl<'a, M> Default for ManagedBufferBuilder<'a, M, ManagedBufferImplDefault<'a, M>>
where
    M: ManagedTypeApi<'a>,
{
    #[inline]
    fn default() -> Self {
        Self::new_from_slice(&[])
    }
}

impl<'a, M, Impl> ManagedBufferBuilder<'a, M, Impl>
where
    M: ManagedTypeApi<'a>,
    Impl: ManagedBufferBuilderImpl<'a, M>,
{
    pub fn into_managed_buffer(self) -> ManagedBuffer<'a, M> {
        self.implementation.into_managed_buffer()
    }

    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.implementation.append_bytes(bytes);
    }

    pub fn append_managed_buffer(&mut self, item: &ManagedBuffer<'a, M>) {
        self.implementation.append_managed_buffer(item);
    }

    /// Converts the input to hex and adds it to the current buffer.
    ///
    /// TODO: consider making the hex conversion part of the VM
    pub fn append_managed_buffer_hex(&mut self, item: &ManagedBuffer<'a, M>) {
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
    pub fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<'a, M>) {
        item.for_each_batch::<BIN_CONVERSION_BUFFER_LEN, _>(|batch| {
            for &byte in batch {
                let ascii_bin_digit = byte_to_binary_digits(byte);
                self.append_bytes(&ascii_bin_digit[..]);
            }
        });
    }
}

impl<'a, M, Impl> NestedEncodeOutput for ManagedBufferBuilder<'a, M, Impl>
where
    M: ManagedTypeApi<'a>,
    Impl: ManagedBufferBuilderImpl<'a, M>,
{
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        T::type_eq::<ManagedBuffer<'a, M>>()
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
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<'a, M>>() {
            self.append_managed_buffer(managed_buffer);
            Ok(())
        } else {
            Err(h.handle_error(EncodeError::UNSUPPORTED_OPERATION))
        }
    }
}

impl<'a, M, Impl> FormatByteReceiver<'a> for ManagedBufferBuilder<'a, M, Impl>
where
    M: ManagedTypeApi<'a>,
    Impl: ManagedBufferBuilderImpl<'a, M>,
{
    type Api = M;

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes)
    }

    fn append_managed_buffer(&mut self, item: &ManagedBuffer<'a, M>) {
        self.append_managed_buffer(item)
    }

    fn append_managed_buffer_lower_hex(&mut self, item: &ManagedBuffer<'a, M>) {
        self.append_managed_buffer_hex(item)
    }

    fn append_managed_buffer_binary(&mut self, item: &ManagedBuffer<'a, M>) {
        self.append_managed_buffer_binary(item)
    }
}

impl<'a, M> FormatBuffer<'a> for ManagedBufferBuilder<'a, M, ManagedBufferImplDefault<'a, M>>
where
    M: ManagedTypeApi<'a>,
{
    fn append_ascii(&mut self, ascii: &[u8]) {
        self.append_bytes(ascii)
    }

    fn append_display<T: SCDisplay<'a>>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_lower_hex<T: SCLowerHex<'a>>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_binary<T: SCBinary<'a>>(&mut self, item: &T) {
        item.fmt(self);
    }

    fn append_codec<T: SCCodec<'a>>(&mut self, item: &T) {
        item.fmt(self);
    }
}
