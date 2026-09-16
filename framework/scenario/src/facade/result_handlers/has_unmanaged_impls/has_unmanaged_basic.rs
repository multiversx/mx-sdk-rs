use multiversx_sc::codec::arrayvec::ArrayVec;

use crate::facade::result_handlers::HasUnmanaged;

impl HasUnmanaged for () {
    type Unmanaged = Self;
}

impl<T: HasUnmanaged> HasUnmanaged for &T {
    type Unmanaged = T::Unmanaged;
}

impl<T: HasUnmanaged> HasUnmanaged for Box<T> {
    type Unmanaged = Box<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for &[T] {
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for Vec<T> {
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<T: HasUnmanaged, const CAP: usize> HasUnmanaged for ArrayVec<T, CAP> {
    type Unmanaged = ArrayVec<T::Unmanaged, CAP>;
}

impl<T: HasUnmanaged> HasUnmanaged for Box<[T]> {
    type Unmanaged = Box<[T::Unmanaged]>;
}

impl HasUnmanaged for String {
    type Unmanaged = Self;
}

impl HasUnmanaged for &'static str {
    type Unmanaged = Self;
}

impl HasUnmanaged for Box<str> {
    type Unmanaged = Self;
}

macro_rules! has_unmanaged_self {
    ($ty:ty) => {
        impl HasUnmanaged for $ty {
            type Unmanaged = Self;
        }
    };
}

has_unmanaged_self!(u8);
has_unmanaged_self!(u16);
has_unmanaged_self!(u32);
has_unmanaged_self!(usize);
has_unmanaged_self!(u64);
has_unmanaged_self!(u128);

has_unmanaged_self!(i8);
has_unmanaged_self!(i16);
has_unmanaged_self!(i32);
has_unmanaged_self!(isize);
has_unmanaged_self!(i64);

has_unmanaged_self!(core::num::NonZeroUsize);
has_unmanaged_self!(bool);
has_unmanaged_self!(f64);

impl<T: HasUnmanaged> HasUnmanaged for Option<T> {
    type Unmanaged = Option<T::Unmanaged>;
}

impl<T: HasUnmanaged, E> HasUnmanaged for Result<T, E> {
    type Unmanaged = Result<T::Unmanaged, E>;
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> HasUnmanaged for ($($name,)+)
            where
                $($name: HasUnmanaged,)+
            {
                type Unmanaged = ($($name::Unmanaged,)+);
            }
        )+
    }
}

tuple_impls! {
    1  => (0 T0)
    2  => (0 T0 1 T1)
    3  => (0 T0 1 T1 2 T2)
    4  => (0 T0 1 T1 2 T2 3 T3)
    5  => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T: HasUnmanaged, const N: usize> HasUnmanaged for [T; N] {
    type Unmanaged = [T::Unmanaged; N];
}
