use crate::{api::ManagedTypeApi, codec::arrayvec::ArrayVec};

use crate::types::{HasManaged, ManagedVec, ManagedVecItem};

impl<M: ManagedTypeApi> HasManaged<M> for () {
    type Managed = Self;
}

impl<M, T> HasManaged<M> for &[T]
where
    M: ManagedTypeApi,
    T: HasManaged<M>,
    T::Managed: ManagedVecItem,
{
    type Managed = ManagedVec<M, T::Managed>;
}

impl<M, T> HasManaged<M> for alloc::vec::Vec<T>
where
    M: ManagedTypeApi,
    T: HasManaged<M>,
    T::Managed: ManagedVecItem,
{
    type Managed = ManagedVec<M, T::Managed>;
}

impl<M: ManagedTypeApi, T: HasManaged<M>, const CAP: usize> HasManaged<M> for ArrayVec<T, CAP> {
    type Managed = ArrayVec<T::Managed, CAP>;
}

impl<M: ManagedTypeApi> HasManaged<M> for &'static str {
    type Managed = Self;
}

macro_rules! has_managed_self {
    ($ty:ty) => {
        impl<M: ManagedTypeApi> HasManaged<M> for $ty {
            type Managed = Self;
        }
    };
}

has_managed_self!(u8);
has_managed_self!(u16);
has_managed_self!(u32);
has_managed_self!(usize);
has_managed_self!(u64);
has_managed_self!(u128);

has_managed_self!(i8);
has_managed_self!(i16);
has_managed_self!(i32);
has_managed_self!(isize);
has_managed_self!(i64);

has_managed_self!(core::num::NonZeroUsize);
has_managed_self!(bool);
has_managed_self!(f64);

impl<M: ManagedTypeApi, T: HasManaged<M>> HasManaged<M> for Option<T> {
    type Managed = Option<T::Managed>;
}

impl<M: ManagedTypeApi, T: HasManaged<M>, E> HasManaged<M> for Result<T, E> {
    type Managed = Result<T::Managed, E>;
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<M: ManagedTypeApi, $($name),+> HasManaged<M> for ($($name,)+)
            where
                $($name: HasManaged<M>,)+
            {
                type Managed = ($($name::Managed,)+);
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

impl<M: ManagedTypeApi, T: HasManaged<M>, const N: usize> HasManaged<M> for [T; N] {
    type Managed = [T::Managed; N];
}
