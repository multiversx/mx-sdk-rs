use crate::{DecodeError, DecodeErrorHandler, NestedDecode, NestedDecodeInput};

/// A nested decode buffer implementation on referenced data.
impl<'a> NestedDecodeInput for &'a [u8] {
    fn remaining_len(&self) -> usize {
        self.len()
    }

    fn peek_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        if into.len() > self.len() {
            return Err(h.handle_error(DecodeError::INPUT_TOO_SHORT));
        }
        let len = into.len();
        into.copy_from_slice(&self[..len]);
        Ok(())
    }

    fn read_into<H>(&mut self, into: &mut [u8], h: H) -> Result<(), H::HandledErr>
    where
        H: DecodeErrorHandler,
    {
        self.peek_into(into, h)?;
        *self = &self[into.len()..];
        Ok(())
    }
}

/// Convenience method, to avoid having to specify type when calling `dep_decode`.
/// Especially useful in the macros.
/// Also checks that the entire slice was used.
/// The input doesn't need to be mutable because we are not changing the underlying data.
pub fn dep_decode_from_byte_slice<T, H>(input: &[u8], h: H) -> Result<T, H::HandledErr>
where
    T: NestedDecode,
    H: DecodeErrorHandler,
{
    let mut_slice = &mut &*input;
    let result = T::dep_decode_or_handle_err(mut_slice, h)?;
    if !mut_slice.is_empty() {
        return Err(h.handle_error(DecodeError::INPUT_TOO_LONG));
    }
    Ok(result)
}
