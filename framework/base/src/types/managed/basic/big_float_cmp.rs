use core::cmp::Ordering;

use crate::api::{use_raw_handle, BigFloatApiImpl, ManagedTypeApi, StaticVarApiImpl};

use super::{BigFloat, BigInt};

impl<'a, M: ManagedTypeApi<'a>> PartialEq for BigFloat<'a, M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bf_cmp(self.handle.clone(), other.handle.clone())
            .is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> Eq for BigFloat<'a, M> {}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd for BigFloat<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a, M: ManagedTypeApi<'a>> Ord for BigFloat<'a, M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        M::managed_type_impl().bf_cmp(self.handle.clone(), other.handle.clone())
    }
}

fn cmp_i64<'a, M: ManagedTypeApi<'a>>(bf: &BigFloat<'a, M>, other: i64) -> Ordering {
    if other == 0 {
        match M::managed_type_impl().bf_sign(bf.handle.clone()) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let new_bf_handle: M::BigFloatHandle =
            use_raw_handle(M::static_var_api_impl().next_handle());
        M::managed_type_impl().bf_set_i64(new_bf_handle.clone(), other);
        M::managed_type_impl().bf_cmp(bf.handle.clone(), new_bf_handle)
    }
}

fn cmp_bi<'a, M: ManagedTypeApi<'a>>(bf: &BigFloat<'a, M>, other: &BigInt<'a, M>) -> Ordering {
    let new_bf_handle: M::BigFloatHandle = use_raw_handle(M::static_var_api_impl().next_handle());
    M::managed_type_impl().bf_set_bi(new_bf_handle.clone(), other.handle.clone());
    M::managed_type_impl().bf_cmp(bf.handle.clone(), new_bf_handle)
}

impl<'a, M: ManagedTypeApi<'a>> PartialEq<i64> for BigFloat<'a, M> {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        cmp_i64(self, *other).is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd<i64> for BigFloat<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
        Some(cmp_i64(self, *other))
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialEq<BigInt<'a, M>> for BigFloat<'a, M> {
    #[inline]
    fn eq(&self, other: &BigInt<'a, M>) -> bool {
        cmp_bi(self, other).is_eq()
    }
}

impl<'a, M: ManagedTypeApi<'a>> PartialOrd<BigInt<'a, M>> for BigFloat<'a, M> {
    #[inline]
    fn partial_cmp(&self, other: &BigInt<'a, M>) -> Option<Ordering> {
        Some(cmp_bi(self, other))
    }
}
