use core::cmp::Ordering;

use crate::api::{const_handles, use_raw_handle, BigIntApi, ManagedTypeApi};

use super::BigInt;

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

fn cmp_i64<M: ManagedTypeApi>(bi: &BigInt<M>, other: i64) -> Ordering {
    let api = M::managed_type_impl();
    if other == 0 {
        match api.bi_sign(bi.handle.clone()) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let big_int_temp_1: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_1);
        M::managed_type_impl().bi_set_int64(big_int_temp_1.clone(), other as i64);
        api.bi_cmp(bi.handle.clone(), big_int_temp_1)
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
