use core::cmp::Ordering;

use crate::api::{BigIntApiImpl, ManagedTypeApi, const_handles};

use super::BigInt;

pub(crate) fn bi_cmp_zero<M>(bi_handle: M::BigIntHandle) -> Ordering
where
    M: ManagedTypeApi,
{
    match M::managed_type_impl().bi_sign(bi_handle) {
        crate::api::Sign::Plus => Ordering::Greater,
        crate::api::Sign::NoSign => Ordering::Equal,
        crate::api::Sign::Minus => Ordering::Less,
    }
}

#[allow(unused)]
pub(crate) fn bi_gt_zero<M>(bi_handle: M::BigIntHandle) -> bool
where
    M: ManagedTypeApi,
{
    bi_cmp_zero::<M>(bi_handle) == Ordering::Greater
}

pub(crate) fn bi_cmp_i64<M>(bi_handle: M::BigIntHandle, other: i64) -> Ordering
where
    M: ManagedTypeApi,
{
    let api = M::managed_type_impl();
    if other == 0 {
        bi_cmp_zero::<M>(bi_handle)
    } else {
        let big_int_temp_1 = BigInt::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
        api.bi_cmp(bi_handle, big_int_temp_1)
    }
}
