use crate::{
    api::{const_handles, use_raw_handle, BigIntApiImpl, ManagedTypeApi, StaticVarApiImpl},
    types::{BigUint, ManagedRef},
};

/// Decimals are represented as usize. This type is also used as variable decimals.
pub type NumDecimals = usize;

/// Implemented by all decimal types usable in `ManagedDecimal`.
pub trait Decimals {
    /// Number of decimals as variable.
    fn num_decimals(&self) -> NumDecimals;

    /// 10^num_decimals, represented as a `BigUint`.
    fn scaling_factor<M: ManagedTypeApi>(&self) -> ManagedRef<'static, M, BigUint<M>> {
        scaling_factor(self.num_decimals())
    }
}

impl Decimals for NumDecimals {
    fn num_decimals(&self) -> NumDecimals {
        *self
    }
}

/// Zero-sized constant number of decimals.
///
/// Ideal if the number of decimals is known at compile time.
#[derive(Clone, Debug)]
pub struct ConstDecimals<const DECIMALS: NumDecimals>;

impl<const DECIMALS: NumDecimals> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS
    }

    fn scaling_factor<M: ManagedTypeApi>(&self) -> ManagedRef<'static, M, BigUint<M>> {
        scaling_factor(self.num_decimals())
    }
}

fn scaling_factor<M: ManagedTypeApi>(
    num_decimals: NumDecimals,
) -> ManagedRef<'static, M, BigUint<M>> {
    let handle: M::BigIntHandle =
        use_raw_handle(const_handles::get_scaling_factor_handle(num_decimals));

    if !M::static_var_api_impl().is_scaling_factor_cached(num_decimals) {
        cache_scaling_factor::<M>(handle.clone(), num_decimals);
        M::static_var_api_impl().set_scaling_factor_cached(num_decimals);
    }

    unsafe { ManagedRef::<'static, M, BigUint<M>>::wrap_handle(handle) }
}

fn cache_scaling_factor<M: ManagedTypeApi>(handle: M::BigIntHandle, num_decimals: NumDecimals) {
    let temp1: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_1);
    let temp2: M::BigIntHandle = use_raw_handle(const_handles::BIG_INT_TEMPORARY_2);
    let api = M::managed_type_impl();
    api.bi_set_int64(temp1.clone(), 10);
    api.bi_set_int64(temp2.clone(), num_decimals as i64);
    api.bi_pow(handle, temp1, temp2);
}
