use crate::{DecodeError, NestedDecode, NestedDecodeInput};

/// A nested decode buffer implementation on referenced data.
impl<'a> NestedDecodeInput for &'a [u8] {
    fn remaining_len(&self) -> usize {
        self.len()
    }

    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
        if into.len() > self.len() {
            return Err(DecodeError::INPUT_TOO_SHORT);
        }
        let len = into.len();
        into.copy_from_slice(&self[..len]);
        *self = &self[len..];
        Ok(())
    }

    fn read_into_or_exit<ExitCtx: Clone>(
        &mut self,
        into: &mut [u8],
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) {
        if into.len() > self.len() {
            exit(c, DecodeError::INPUT_TOO_SHORT);
        }
        let len = into.len();
        into.copy_from_slice(&self[..len]);
        *self = &self[len..];
    }
}

/// Convenience method, to avoid having to specify type when calling `dep_decode`.
/// Especially useful in the macros.
/// Also checks that the entire slice was used.
/// The input doesn't need to be mutable because we are not changing the underlying data.
pub fn dep_decode_from_byte_slice<D: NestedDecode>(input: &[u8]) -> Result<D, DecodeError> {
    let mut_slice = &mut &*input;
    let result = D::dep_decode(mut_slice);
    if !mut_slice.is_empty() {
        return Err(DecodeError::INPUT_TOO_LONG);
    }
    result
}

pub fn dep_decode_from_byte_slice_or_exit<D: NestedDecode, ExitCtx: Clone>(
    input: &[u8],
    c: ExitCtx,
    exit: fn(ExitCtx, DecodeError) -> !,
) -> D {
    let mut_slice = &mut &*input;
    let result = D::dep_decode_or_exit(mut_slice, c.clone(), exit);
    if !mut_slice.is_empty() {
        exit(c, DecodeError::INPUT_TOO_LONG);
    }
    result
}
