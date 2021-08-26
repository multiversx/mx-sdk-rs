use core::cmp::Ordering;

use crate::api::ManagedTypeApi;

use super::BigInt;

impl<M: ManagedTypeApi> PartialEq for BigInt<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.api.bi_cmp(self.handle, other.handle).is_eq()
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
        self.api.bi_cmp(self.handle, other.handle)
    }
}

fn cmp_i64<M: ManagedTypeApi>(bi: &BigInt<M>, other: i64) -> Ordering {
    if other == 0 {
        match bi.api.bi_sign(bi.handle) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        bi.api.bi_cmp(bi.handle, bi.api.bi_new(other))
    }
}

impl<M: ManagedTypeApi> PartialEq<i64> for BigInt<M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<M: ManagedTypeApi> PartialOrd<i64> for BigInt<M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}
