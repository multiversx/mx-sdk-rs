use core::cmp::Ordering;

use crate::api::ManagedTypeApi;

use crate::types::cast_to_i64::cast_to_i64;

use super::BigUint;

impl<M: ManagedTypeApi> PartialEq for BigUint<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<M: ManagedTypeApi> Eq for BigUint<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigUint<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigUint<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

macro_rules! partial_eq_and_ord {
    ($small_int_type:ident) => {
        impl<M: ManagedTypeApi> PartialEq<$small_int_type> for BigUint<M> {
            #[inline]
            fn eq(&self, other: &$small_int_type) -> bool {
                self.value.eq(&cast_to_i64::<M, _>(*other))
            }
        }

        impl<M: ManagedTypeApi> PartialOrd<$small_int_type> for BigUint<M> {
            #[inline]
            fn partial_cmp(&self, other: &$small_int_type) -> Option<Ordering> {
                self.value.partial_cmp(&cast_to_i64::<M, _>(*other))
            }
        }
    };
}

partial_eq_and_ord! {i32}
partial_eq_and_ord! {i64}
partial_eq_and_ord! {u32}
partial_eq_and_ord! {u64}
