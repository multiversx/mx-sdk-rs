use crate::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{BigUint, Decimals, ManagedDecimal, NumDecimals},
};

use core::ops::{Deref, Div, DivAssign, Sub};

use super::ConstDecimals;

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> DivAssign<&ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn div_assign(&mut self, rhs: &ManagedDecimal<M, D2>) {
        self.data *= rhs.scaling_factor().deref();
        self.data /= &rhs.data;
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> DivAssign<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    #[inline]
    fn div_assign(&mut self, rhs: ManagedDecimal<M, D2>) {
        self.div_assign(&rhs);
    }
}

impl<M: ManagedTypeApi, D: Decimals> Div<NumDecimals> for ManagedDecimal<M, D> {
    type Output = Self;

    fn div(self, other: NumDecimals) -> Self::Output {
        ManagedDecimal {
            data: self.data / BigUint::from(other),
            decimals: self.decimals,
        }
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> Div<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
where
    D1: Sub<D2>,
    <D1 as Sub<D2>>::Output: Decimals,
{
    type Output = ManagedDecimal<M, <D1 as Sub<D2>>::Output>;

    fn div(self, other: ManagedDecimal<M, D2>) -> Self::Output {
        ManagedDecimal {
            data: self.data / other.data,
            decimals: self.decimals - other.decimals,
        }
    }
}

// var + const
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Div<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn div(self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self / rhs.into_var_decimals()
    }
}

// const + var
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Div<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn div(self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() / rhs
    }
}
