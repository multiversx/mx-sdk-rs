use core::{cmp::Ordering, convert::TryInto};

use crate::{
    api::{const_handles, BigIntApiImpl, ManagedTypeApi},
    types::ManagedType,
};

use super::{cast_to_i64::cast_to_i64, BigInt};

pub(crate) fn cmp_i64<M, B>(bi: &B, other: i64) -> Ordering
where
    M: ManagedTypeApi,
    B: ManagedType<M, OwnHandle = M::BigIntHandle>,
{
    let api = M::managed_type_impl();
    if other == 0 {
        match api.bi_sign(bi.get_handle()) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let big_int_temp_1 = BigInt::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
        api.bi_cmp(bi.get_handle(), big_int_temp_1)
    }
}

pub(crate) fn cmp_conv_i64<M, B, T>(bi: &B, other: T) -> Ordering
where
    M: ManagedTypeApi,
    B: ManagedType<M, OwnHandle = M::BigIntHandle>,
    T: TryInto<i64>,
{
    cmp_i64(bi, cast_to_i64::<M, _>(other))
}
