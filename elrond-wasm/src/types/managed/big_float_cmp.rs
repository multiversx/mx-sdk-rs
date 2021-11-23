use core::cmp::Ordering;

use crate::api::ManagedTypeApi;

use super::{BigFloat, BigInt};

impl<M: ManagedTypeApi> PartialEq for BigFloat<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let api = M::instance();
        api.bf_cmp(self.handle, other.handle).is_eq()
    }
}

impl<M: ManagedTypeApi> Eq for BigFloat<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigFloat<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigFloat<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let api = M::instance();
        api.bf_cmp(self.handle, other.handle)
    }
}

fn cmp_i64<M: ManagedTypeApi>(bf: &BigFloat<M>, other: i64) -> Ordering {
    let api = M::instance();
    if other == 0 {
        match api.bf_sign(bf.handle) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let new_bf_handle = api.bf_new_zero();
        api.bf_set_i64(new_bf_handle, other);
        api.bf_cmp(bf.handle, new_bf_handle)
    }
}

fn cmp_bi<M: ManagedTypeApi>(bf: &BigFloat<M>, other: &BigInt<M>) -> Ordering {
    let api = M::instance();
    let new_bf_handle = api.bf_new_zero();
    api.bf_set_bi(new_bf_handle, other.handle);
    api.bf_cmp(bf.handle, new_bf_handle)
}

impl<M: ManagedTypeApi> PartialEq<i64> for BigFloat<M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<M: ManagedTypeApi> PartialOrd<i64> for BigFloat<M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}

impl<M: ManagedTypeApi> PartialEq<BigInt<M>> for BigFloat<M> {
    #[inline]
    fn eq(&self, other: &BigInt<M>) -> bool {
        cmp_bi(self, other).is_eq()
    }
}

impl<M: ManagedTypeApi> PartialOrd<BigInt<M>> for BigFloat<M> {
    #[inline]
    fn partial_cmp(&self, other: &BigInt<M>) -> Option<Ordering> {
        Some(cmp_bi(self, other))
    }
}
