pub use crate::codec_err::DecodeError;
use crate::TryStaticCast;

/// Trait that allows deserializing objects from a buffer.
pub trait NestedDecodeInput {
    /// The remaining length of the input data.
    fn remaining_len(&self) -> usize;

    /// True if all data from the buffer has already been used.
    fn is_depleted(&self) -> bool {
        self.remaining_len() == 0
    }

    /// Read the exact number of bytes required to fill the given buffer.
    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError>;

    /// Read the exact number of bytes required to fill the given buffer.
    /// Exit early if there are not enough bytes to fill the result.
    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    );

    #[inline]
    fn read_specialized<T: TryStaticCast>(&mut self) -> Result<Option<T>, DecodeError> {
        Ok(None)
    }

    #[inline]
    fn read_specialized_or_exit<T: TryStaticCast, ExitCtx: Clone>(
        &mut self,
        _c: ExitCtx,
        _exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Option<T> {
        None
    }

    /// Read a single byte from the input.
    fn read_byte(&mut self) -> Result<u8, DecodeError> {
        let mut buf = [0u8];
        self.read_into(&mut buf[..])?;
        Ok(buf[0])
    }

    /// Read a single byte from the input.
    fn read_byte_or_exit<ExitCtx: Clone>(
        &mut self,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> u8 {
        let mut buf = [0u8];
        self.read_into_or_exit(&mut buf[..], c, exit);
        buf[0]
    }
}
