use core::cmp::Ordering;

use crate::{
    api::{BigIntApiImpl, ManagedTypeApi},
    types::ManagedType,
};

use super::{BigInt, big_num_cmp::bi_cmp_i64};

impl<M: ManagedTypeApi> PartialEq for BigInt<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bi_cmp(self.handle.clone(), other.handle.clone())
            .is_eq()
    }
}

impl<M: ManagedTypeApi> Eq for BigInt<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigInt<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        M::managed_type_impl().bi_cmp(self.handle.clone(), other.handle.clone())
    }
}

macro_rules! partial_eq_and_ord {
    ($small_int_type:ident) => {
        impl<M: ManagedTypeApi> PartialEq<$small_int_type> for BigInt<M> {
            #[inline]
            fn eq(&self, other: &$small_int_type) -> bool {
                bi_cmp_i64::<M>(self.get_handle(), *other as i64).is_eq()
            }
        }

        impl<M: ManagedTypeApi> PartialOrd<$small_int_type> for BigInt<M> {
            #[inline]
            fn partial_cmp(&self, other: &$small_int_type) -> Option<Ordering> {
                Some(bi_cmp_i64::<M>(self.get_handle(), *other as i64))
            }
        }
    };
}

partial_eq_and_ord! {i32}
partial_eq_and_ord! {i64}
partial_eq_and_ord! {u32}
partial_eq_and_ord! {u64}
