use crate::{TopDecodeMulti, TopEncodeMulti};

/// Signals that we can safely serialize `Self` in order to obtain a `T` on the other size.
pub trait EquivalentArgument<T>: TopEncodeMulti
where
    T: TopDecodeMulti,
{
}

impl<T> EquivalentArgument<T> for T where T: TopEncodeMulti + TopDecodeMulti {}

impl<'a, T> EquivalentArgument<T> for &'a T
where
    T: TopDecodeMulti,
    &'a T: TopEncodeMulti,
{
}
impl<'a, T> EquivalentArgument<&'a T> for T
where
    &'a T: TopDecodeMulti,
    T: TopEncodeMulti,
{
}

// Unsigned integer types: we can serialize at a smaller capacity and deserialize at a larger capacity.

impl EquivalentArgument<u64> for u32 {}
impl EquivalentArgument<usize> for u32 {}

impl EquivalentArgument<u64> for usize {}
impl EquivalentArgument<u32> for usize {}

impl EquivalentArgument<u64> for u16 {}
impl EquivalentArgument<u32> for u16 {}
impl EquivalentArgument<usize> for u16 {}

impl EquivalentArgument<u64> for u8 {}
impl EquivalentArgument<u32> for u8 {}
impl EquivalentArgument<usize> for u8 {}
impl EquivalentArgument<u16> for u8 {}

// Signed, the same.

impl EquivalentArgument<i64> for i32 {}
impl EquivalentArgument<isize> for i32 {}

impl EquivalentArgument<i64> for isize {}
impl EquivalentArgument<i32> for isize {}

impl EquivalentArgument<i64> for i16 {}
impl EquivalentArgument<i32> for i16 {}
impl EquivalentArgument<isize> for i16 {}

impl EquivalentArgument<i64> for i8 {}
impl EquivalentArgument<i32> for i8 {}
impl EquivalentArgument<isize> for i8 {}
impl EquivalentArgument<i16> for i8 {}
