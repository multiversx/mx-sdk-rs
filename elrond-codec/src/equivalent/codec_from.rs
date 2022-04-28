use crate::{TopDecodeMulti, TopEncodeMulti};

/// Signals that after serializing `T`, we can safely deserialize it as `Self`.
pub trait CodecFrom<T>: TopDecodeMulti
where
    T: TopEncodeMulti,
{
}

pub auto trait CodecFromSelf {}

impl<T> CodecFrom<T> for T where T: TopEncodeMulti + TopDecodeMulti + CodecFromSelf {}

impl<'a, T> CodecFrom<&'a T> for T
where
    &'a T: TopEncodeMulti,
    T: TopDecodeMulti,
{
}

// Unsigned integer types: the contract can return a smaller capacity result and and we can interpret it as a larger capacity type.

impl CodecFrom<usize> for u64 {}
impl CodecFrom<u32> for u64 {}
impl CodecFrom<u16> for u64 {}
impl CodecFrom<u8> for u64 {}

impl CodecFrom<usize> for u32 {}
impl CodecFrom<u16> for u32 {}
impl CodecFrom<u8> for u32 {}

impl CodecFrom<u32> for usize {}
impl CodecFrom<u16> for usize {}
impl CodecFrom<u8> for usize {}

impl CodecFrom<u8> for u16 {}

// Signed, the same.

impl CodecFrom<isize> for i64 {}
impl CodecFrom<i32> for i64 {}
impl CodecFrom<i16> for i64 {}
impl CodecFrom<i8> for i64 {}

impl CodecFrom<isize> for i32 {}
impl CodecFrom<i16> for i32 {}
impl CodecFrom<i8> for i32 {}

impl CodecFrom<i32> for isize {}
impl CodecFrom<i16> for isize {}
impl CodecFrom<i8> for isize {}

impl CodecFrom<i8> for i16 {}
