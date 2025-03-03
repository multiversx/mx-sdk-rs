use core::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use crate::{
    api::{const_handles, use_raw_handle, BigIntApiImpl, ManagedTypeApi, StaticVarApiImpl},
    typenum::Unsigned,
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
#[derive(Clone, Default, Debug)]
pub struct ConstDecimals<DECIMALS: Unsigned> {
    _phantom: PhantomData<DECIMALS>,
}

impl<DECIMALS: Unsigned> ConstDecimals<DECIMALS> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<DECIMALS: Unsigned> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS::to_usize()
    }

    fn scaling_factor<M: ManagedTypeApi>(&self) -> ManagedRef<'static, M, BigUint<M>> {
        scaling_factor(self.num_decimals())
    }
}

impl<DEC1, DEC2> Add<ConstDecimals<DEC2>> for ConstDecimals<DEC1>
where
    DEC1: Unsigned,
    DEC2: Unsigned,
    DEC1: Add<DEC2>,
    <DEC1 as Add<DEC2>>::Output: Unsigned,
{
    type Output = ConstDecimals<<DEC1 as Add<DEC2>>::Output>;
    fn add(self, _rhs: ConstDecimals<DEC2>) -> Self::Output {
        ConstDecimals::new()
    }
}

impl<DEC1, DEC2> Sub<ConstDecimals<DEC2>> for ConstDecimals<DEC1>
where
    DEC1: Unsigned,
    DEC2: Unsigned,
    DEC1: Sub<DEC2>,
    <DEC1 as Sub<DEC2>>::Output: Unsigned,
{
    type Output = ConstDecimals<<DEC1 as Sub<DEC2>>::Output>;
    fn sub(self, _rhs: ConstDecimals<DEC2>) -> Self::Output {
        ConstDecimals::new()
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
