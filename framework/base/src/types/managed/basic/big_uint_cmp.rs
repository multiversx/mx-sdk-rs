use crate::types::managed::basic::big_num_cmp::cmp_conv_i64;
use core::cmp::Ordering;

use crate::api::{BigIntApiImpl, ManagedTypeApi};

use super::BigUint;

impl<'a, M: ManagedTypeApi<'a>> PartialEq for BigUint<'a, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bi_cmp(self.handle.clone(), other.handle.clone())
            .is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> Eq for BigUint<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd for BigUint<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M: ManagedTypeApi<'a>> Ord for BigUint<'a, M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        M::managed_type_impl().bi_cmp(self.handle.clone(), other.handle.clone())
    }
}

macro_rules! partial_eq_and_ord {
    ($small_int_type:ident) => {
        impl<'a, M: ManagedTypeApi<'a>> PartialEq<$small_int_type> for BigUint<'a, M> {
            #[inline]
            fn eq(&self, other: &$small_int_type) -> bool {
                cmp_conv_i64(self, *other).is_eq()
            }
        }

        impl<'a, M: ManagedTypeApi<'a>> PartialOrd<$small_int_type> for BigUint<'a, M> {
            #[inline]
            fn partial_cmp(&self, other: &$small_int_type) -> Option<Ordering> {
                Some(cmp_conv_i64(self, *other))
            }
        }
    };
}

partial_eq_and_ord! {i32}
partial_eq_and_ord! {i64}
partial_eq_and_ord! {u32}
partial_eq_and_ord! {u64}
