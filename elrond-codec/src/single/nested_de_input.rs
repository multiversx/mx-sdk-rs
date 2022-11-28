pub use crate::codec_err::DecodeError;
use crate::{DecodeErrorHandler, TryStaticCast};

/// Trait that allows deserializing objects from a buffer.
pub trait NestedDecodeInput {
    /// The remaining length of the input data.
    fn remaining_len(&self) -> usize;

    /// True if all data from the buffer has already been used.
    fn is_depleted(&self) -> bool {
        self.remaining_len() == 0
    }

    /// Read the exact number of bytes required to fill the given buffer, without consuming the underlying bytes.
    ///
    /// Will fail is not enough bytes left in buffer.
    fn peek_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler;

    /// Read & consume the exact number of bytes required to fill the given buffer.
    ///
    /// Will fail is not enough bytes left in buffer.
    fn read_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler;

    #[inline]
    fn supports_specialized_type<T: TryStaticCast>() -> bool {
        false
    }

    fn read_specialized<T, C, H>(&mut self, _context: C, h: H) -> Result<T, H::HandledErr>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        H: DecodeErrorHandler,
    {
        Err(h.handle_error(DecodeError::UNSUPPORTED_OPERATION))
    }

    /// Read a single byte from the input.
    fn read_byte<H>(&mut self, h: H) -> Result<u8, H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        let mut buf = [0u8];
        self.read_into(&mut buf[..], h)?;
        Ok(buf[0])
    }
}
