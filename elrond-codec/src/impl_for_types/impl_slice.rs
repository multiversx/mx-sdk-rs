use crate::codec_err::{DecodeError, EncodeError};
use crate::nested_de::NestedDecode;
use crate::nested_ser::NestedEncode;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_de::TopDecode;
use crate::top_de_input::TopDecodeInput;
use crate::top_ser::TopEncode;
use crate::top_ser_output::TopEncodeOutput;
use crate::{vec_into_boxed_slice, TypeInfo};
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Adds the concantenated encoded contents of a slice to an output buffer,
/// without serializing the slice length.
/// Byte slice is treated separately, via direct transmute.
pub fn dep_encode_slice_contents<T: NestedEncode, O: NestedEncodeOutput>(
    slice: &[T],
    dest: &mut O,
) -> Result<(), EncodeError> {
    match T::TYPE_INFO {
        TypeInfo::U8 => {
            // cast &[T] to &[u8]
            let slice: &[u8] =
                unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
            dest.write(slice);
        },
        _ => {
            for x in slice {
                x.dep_encode(dest)?;
            }
        },
    }
    Ok(())
}

pub fn dep_encode_slice_contents_or_exit<T, O, ExitCtx>(
    slice: &[T],
    dest: &mut O,
    c: ExitCtx,
    exit: fn(ExitCtx, EncodeError) -> !,
) where
    T: NestedEncode,
    O: NestedEncodeOutput,
    ExitCtx: Clone,
{
    match T::TYPE_INFO {
        TypeInfo::U8 => {
            // cast &[T] to &[u8]
            let slice: &[u8] =
                unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
            dest.write(slice);
        },
        _ => {
            for x in slice {
                x.dep_encode_or_exit(dest, c.clone(), exit);
            }
        },
    }
}

impl<T: NestedEncode> TopEncode for &[T] {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        match T::TYPE_INFO {
            TypeInfo::U8 => {
                // transmute to &[u8]
                // save directly, without passing through the buffer
                let slice: &[u8] =
                    unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
                output.set_slice_u8(slice);
            },
            _ => {
                // only using `dep_encode_slice_contents` for non-u8,
                // because it always appends to the buffer,
                // which is not necessary above
                let mut buffer = Vec::<u8>::new();
                dep_encode_slice_contents(self, &mut buffer)?;
                output.set_slice_u8(&buffer[..]);
            },
        }
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match T::TYPE_INFO {
            TypeInfo::U8 => {
                // transmute to &[u8]
                // save directly, without passing through the buffer
                let slice: &[u8] =
                    unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
                output.set_slice_u8(slice);
            },
            _ => {
                // only using `dep_encode_slice_contents` for non-u8,
                // because it always appends to the buffer,
                // which is not necessary above
                let mut buffer = Vec::<u8>::new();
                for x in *self {
                    x.dep_encode_or_exit(&mut buffer, c.clone(), exit);
                }
                output.set_slice_u8(&buffer[..]);
            },
        }
    }
}

impl<T: NestedEncode> TopEncode for Box<[T]> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_ref().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().top_encode_or_exit(output, c, exit);
    }
}

// Allowed to implement this because [T] cannot implement NestedDecode, being ?Sized.
impl<T: NestedDecode> TopDecode for Box<[T]> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
            Ok(cast_bytes)
        } else {
            let vec = Vec::<T>::top_decode(input)?;
            Ok(vec_into_boxed_slice(vec))
        }
    }

    /// Quick exit for any of the contained types
    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let TypeInfo::U8 = T::TYPE_INFO {
            let bytes = input.into_boxed_slice_u8();
            let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
            cast_bytes
        } else {
            let vec = Vec::<T>::top_decode_or_exit(input, c, exit);
            vec_into_boxed_slice(vec)
        }
    }
}

impl<T: NestedEncode> NestedEncode for &[T] {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        // push size
        self.len().dep_encode(dest)?;
        // actual data
        dep_encode_slice_contents(self, dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        // push size
        self.len().dep_encode_or_exit(dest, c.clone(), exit);
        // actual data
        dep_encode_slice_contents_or_exit(self, dest, c, exit);
    }
}

impl<T: NestedEncode> NestedEncode for Box<[T]> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_ref().dep_encode(dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().dep_encode_or_exit(dest, c, exit);
    }
}

// TODO: NestedDecode for Box<[T]> missing
