use crate::codec_err::EncodeError;
use crate::nested_ser::{dep_encode_slice_contents, NestedEncode};
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_encode_from_no_err;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait TopEncodeNoErr: Sized {
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O);
}

/// Quick encoding of a type that never fails on encoding.
pub fn top_encode_no_err<T: TopEncodeNoErr>(obj: &T) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode_no_err(&mut bytes);
    bytes
}

pub trait TopEncode: Sized {
    // !INTERNAL USE ONLY!
    #[doc(hidden)]
    const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

    /// Attempt to serialize the value to ouput.
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError>;

    /// Version of `top_decode` that exits quickly in case of error.
    /// Its purpose is to create smaller bytecode implementations
    /// in cases where the application is supposed to exit directly on decode error.
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self.top_encode(output) {
            Ok(v) => v,
            Err(e) => exit(c, e),
        }
    }
}

pub fn top_encode_from_nested<T, O>(obj: &T, output: O) -> Result<(), EncodeError>
where
    O: TopEncodeOutput,
    T: NestedEncode,
{
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode(&mut bytes)?;
    output.set_slice_u8(&bytes[..]);
    Ok(())
}

pub fn top_encode_from_nested_or_exit<T, O, ExitCtx>(
    obj: &T,
    output: O,
    c: ExitCtx,
    exit: fn(ExitCtx, EncodeError) -> !,
) where
    O: TopEncodeOutput,
    T: NestedEncode,
    ExitCtx: Clone,
{
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode_or_exit(&mut bytes, c, exit);
    output.set_slice_u8(&bytes[..]);
}

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode(&mut bytes)?;
    Ok(bytes)
}

impl TopEncodeNoErr for () {
    #[inline]
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
        output.set_unit();
    }
}

top_encode_from_no_err! {(), TypeInfo::Unit}

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

impl<T: TopEncode> TopEncode for &T {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        (*self).top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        (*self).top_encode_or_exit(output, c, exit);
    }
}

impl TopEncode for &str {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(self.as_bytes());
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        output.set_slice_u8(self.as_bytes());
    }
}

impl<T: NestedEncode> TopEncode for Vec<T> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_slice().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_slice().top_encode_or_exit(output, c, exit);
    }
}

impl TopEncode for String {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_bytes().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().top_encode_or_exit(output, c, exit);
    }
}

impl TopEncode for Box<str> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_ref().as_bytes().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().as_bytes().top_encode_or_exit(output, c, exit);
    }
}

impl TopEncodeNoErr for bool {
    fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
        // only using signed because this one is implemented in Arwen, unsigned is not
        // TODO: change to set_u64
        output.set_i64(if *self { 1i64 } else { 0i64 });
    }
}

top_encode_from_no_err! {bool, TypeInfo::Bool}

impl<T: NestedEncode> TopEncode for Option<T> {
    /// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
    /// to allow disambiguation between e.g. Some(0) and None.
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        match self {
            Some(v) => {
                let mut buffer = Vec::<u8>::new();
                buffer.push_byte(1u8);
                v.dep_encode(&mut buffer)?;
                output.set_slice_u8(&buffer[..]);
            },
            None => {
                output.set_slice_u8(&[]);
            },
        }
        Ok(())
    }

    /// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
    /// to allow disambiguation between e.g. Some(0) and None.
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            Some(v) => {
                let mut buffer = Vec::<u8>::new();
                buffer.push_byte(1u8);
                v.dep_encode_or_exit(&mut buffer, c, exit);
                output.set_slice_u8(&buffer[..]);
            },
            None => {
                output.set_slice_u8(&[]);
            },
        }
    }
}

impl<T: TopEncode> TopEncode for Box<T> {
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

impl TopEncode for NonZeroUsize {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.get().top_encode(output)
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.get().top_encode_or_exit(output, c, exit);
    }
}
