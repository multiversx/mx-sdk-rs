use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::EncodeError;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::TypeInfo;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait NestedEncodeNoErr: Sized {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O);
}

/// Trait that allows zero-copy write of value-references to slices in LE format.
///
/// Implementations should override `using_top_encoded` for value types and `dep_encode` and `size_hint` for allocating types.
/// Wrapper types should override all methods.
pub trait NestedEncode: Sized {
    // !INTERNAL USE ONLY!
    // This const helps SCALE to optimize the encoding/decoding by doing fake specialization.
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// NestedEncode to output, using the format of an object nested inside another structure.
    /// Does not provide compact version.
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self.dep_encode(dest) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

/// Convenience function for getting an object nested-encoded to a Vec<u8> directly.
pub fn dep_encode_to_vec<T: NestedEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode(&mut bytes)?;
    Ok(bytes)
}

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

impl NestedEncodeNoErr for () {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, _: &mut O) {}
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

impl<T: NestedEncode> NestedEncode for &T {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        (*self).dep_encode(dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        (*self).dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedEncode for &str {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_bytes().dep_encode(dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().dep_encode_or_exit(dest, c, exit);
    }
}

impl<T: NestedEncode> NestedEncode for Vec<T> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_slice().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedEncode for String {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_bytes().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedEncode for Box<str> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_ref().as_bytes().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().as_bytes().dep_encode_or_exit(dest, c, exit);
    }
}

// No reversing needed for u8, because it is a single byte.
impl NestedEncodeNoErr for u8 {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
        dest.push_byte(*self as u8);
    }
}

impl<T: NestedEncode> NestedEncode for Option<T> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            Some(v) => {
                dest.push_byte(1u8);
                v.dep_encode(dest)
            },
            None => {
                dest.push_byte(0u8);
                Ok(())
            },
        }
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            Some(v) => {
                dest.push_byte(1u8);
                v.dep_encode_or_exit(dest, c, exit);
            },
            None => {
                dest.push_byte(0u8);
            },
        }
    }
}

impl<T: NestedEncode> NestedEncode for Box<T> {
    #[inline(never)]
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

impl NestedEncode for NonZeroUsize {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.get().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.get().dep_encode_or_exit(dest, c, exit);
    }
}
