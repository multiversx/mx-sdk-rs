use core::cmp::Ordering;

use crate::api::{BigIntApiImpl, ManagedTypeApi};

use super::{big_num_cmp::cmp_i64, BigInt};

impl<'a, M: ManagedTypeApi<'a>> PartialEq for BigInt<'a, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bi_cmp(self.handle.clone(), other.handle.clone())
            .is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> Eq for BigInt<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd for BigInt<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M: ManagedTypeApi<'a>> Ord for BigInt<'a, M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        M::managed_type_impl().bi_cmp(self.handle.clone(), other.handle.clone())
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialEq<i64> for BigInt<'a, M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd<i64> for BigInt<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}
