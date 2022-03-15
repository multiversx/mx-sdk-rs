use crate::{TopDecodeMulti, TopEncodeMulti};

/// Signals that after serializing `T`, we can safely deserialize it as `Self`.
pub trait EquivalentResult<T>: TopDecodeMulti
where
    T: TopEncodeMulti,
{
}

impl<T> EquivalentResult<T> for T where T: TopEncodeMulti + TopDecodeMulti {}

// Unsigned integer types: the contract can return a smaller capacity result and and we can interpret it as a larger capacity type.

impl EquivalentResult<usize> for u64 {}
impl EquivalentResult<u32> for u64 {}
impl EquivalentResult<u16> for u64 {}
impl EquivalentResult<u8> for u64 {}

impl EquivalentResult<usize> for u32 {}
impl EquivalentResult<u16> for u32 {}
impl EquivalentResult<u8> for u32 {}

impl EquivalentResult<u32> for usize {}
impl EquivalentResult<u16> for usize {}
impl EquivalentResult<u8> for usize {}

impl EquivalentResult<u8> for u16 {}

// Signed, the same.

impl EquivalentResult<isize> for i64 {}
impl EquivalentResult<i32> for i64 {}
impl EquivalentResult<i16> for i64 {}
impl EquivalentResult<i8> for i64 {}

impl EquivalentResult<isize> for i32 {}
impl EquivalentResult<i16> for i32 {}
impl EquivalentResult<i8> for i32 {}

impl EquivalentResult<i32> for isize {}
impl EquivalentResult<i16> for isize {}
impl EquivalentResult<i8> for isize {}

impl EquivalentResult<i8> for i16 {}
