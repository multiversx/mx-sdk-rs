use core::{cmp::Ordering, convert::TryInto};

use crate::api::{const_handles, BigIntApi, ManagedTypeApi};

use super::{BigInt, BigUint};

impl<M: ManagedTypeApi> PartialEq for BigUint<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bi_cmp(self.handle.clone(), other.handle.clone())
            .is_eq()
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
        M::managed_type_impl().bi_cmp(self.handle.clone(), other.handle.clone())
    }
}

fn cmp<M: ManagedTypeApi, T>(bi: &BigUint<M>, other: T) -> Ordering
where
    T: TryInto<i64> + PartialEq<T> + From<u8>,
{
    let api = M::managed_type_impl();
    if other == 0u8.into() {
        match api.bi_sign(bi.handle.clone()) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let big_int_temp_1 = BigInt::<M>::make_temp(const_handles::BIG_INT_TEMPORARY_1, other);
        api.bi_cmp(bi.handle.clone(), big_int_temp_1)
    }
}

macro_rules! partial_eq_and_ord {
    ($small_int_type:ident) => {
        impl<M: ManagedTypeApi> PartialEq<$small_int_type> for BigUint<M> {
            #[inline]
            fn eq(&self, other: &$small_int_type) -> bool {
                cmp(self, *other).is_eq()
            }
        }

        impl<M: ManagedTypeApi> PartialOrd<$small_int_type> for BigUint<M> {
            #[inline]
            fn partial_cmp(&self, other: &$small_int_type) -> Option<Ordering> {
                Some(cmp(self, *other))
            }
        }
    };
}

partial_eq_and_ord! {i32}
partial_eq_and_ord! {i64}
partial_eq_and_ord! {u32}
partial_eq_and_ord! {u64}
