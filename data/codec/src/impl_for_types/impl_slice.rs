use crate::{
    vec_into_boxed_slice, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use alloc::{boxed::Box, vec::Vec};

/// Adds the concantenated encoded contents of a slice to an output buffer,
/// without serializing the slice length.
/// Byte slice is treated separately, via direct transmute.
pub fn dep_encode_slice_contents<T, O, H>(
    slice: &[T],
    dest: &mut O,
    h: H,
) -> Result<(), H::HandledErr>
where
    T: NestedEncode,
    O: NestedEncodeOutput,
    H: EncodeErrorHandler,
{
    T::if_u8(
        dest,
        |dest| {
            // cast &[T] to &[u8]
            let slice: &[u8] =
                unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
            dest.write(slice);
            Ok(())
        },
        |dest| {
            for x in slice {
                x.dep_encode_or_handle_err(dest, h)?;
            }
            Ok(())
        },
    )
}

impl<T: NestedEncode> TopEncode for &[T] {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        T::if_u8(
            output,
            |output| {
                // transmute to &[u8]
                // save directly, without passing through the buffer
                let slice: &[u8] =
                    unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
                output.set_slice_u8(slice);
                Ok(())
            },
            |output| {
                // only using `dep_encode_slice_contents` for non-u8,
                // because it always appends to the buffer,
                // which is not necessary above
                let mut buffer = output.start_nested_encode();
                dep_encode_slice_contents(self, &mut buffer, h)?;
                output.finalize_nested_encode(buffer);
                Ok(())
            },
        )
    }
}

impl<T: NestedEncode> TopEncode for Box<[T]> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().top_encode_or_handle_err(output, h)
    }
}

// Allowed to implement this because [T] cannot implement NestedDecode, being ?Sized.
impl<T: NestedDecode> TopDecode for Box<[T]> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        T::if_u8(
            input,
            |input| {
                let bytes = input.into_boxed_slice_u8();
                let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
                Ok(cast_bytes)
            },
            |input| {
                let vec = Vec::<T>::top_decode_or_handle_err(input, h)?;
                Ok(vec_into_boxed_slice(vec))
            },
        )
    }
}

impl<T: NestedEncode> NestedEncode for &[T] {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        // push size
        self.len().dep_encode_or_handle_err(dest, h)?;
        // actual data
        dep_encode_slice_contents(self, dest, h)
    }
}

impl<T: NestedEncode> NestedEncode for Box<[T]> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().dep_encode_or_handle_err(dest, h)
    }
}

// TODO: NestedDecode for Box<[T]> missing
