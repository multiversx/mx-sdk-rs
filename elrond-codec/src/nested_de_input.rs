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
    fn read_specialized<T, C, F>(&mut self, _context: C, else_deser: F) -> Result<T, DecodeError>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<T, DecodeError>,
    {
        else_deser(self)
    }

    #[inline]
    fn read_specialized_or_exit<T, C, ExitCtx, F>(
        &mut self,
        _context: C,
        c: ExitCtx,
        _exit: fn(ExitCtx, DecodeError) -> !,
        else_deser: F,
    ) -> T
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self, ExitCtx) -> T,
        ExitCtx: Clone,
    {
        else_deser(self, c)
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
