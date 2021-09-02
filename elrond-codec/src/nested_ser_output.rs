use alloc::vec::Vec;

use crate::{EncodeError, TryStaticCast};

/// Trait that allows appending bytes.
/// Used especially by the NestedEncode trait to output data.
///
/// In principle it can be anything, but in practice
/// we only keep 1 implementation, which is Vec<u8>.
/// This is to avoid code duplication by monomorphization.
pub trait NestedEncodeOutput {
    /// Write to the output.
    fn write(&mut self, bytes: &[u8]);

    /// Write a single byte to the output.
    fn push_byte(&mut self, byte: u8) {
        self.write(&[byte]);
    }

    #[inline]
    fn push_specialized<T, C, F>(
        &mut self,
        _context: C,
        _value: &T,
        else_serialization: F,
    ) -> Result<(), EncodeError>
    where
        T: TryStaticCast,
        C: TryStaticCast,
        F: FnOnce(&mut Self) -> Result<(), EncodeError>,
    {
        else_serialization(self)
    }
}

impl NestedEncodeOutput for Vec<u8> {
    fn write(&mut self, bytes: &[u8]) {
        self.extend_from_slice(bytes)
    }
}
