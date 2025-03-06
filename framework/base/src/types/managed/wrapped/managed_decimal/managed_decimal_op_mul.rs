use crate::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{Decimals, ManagedDecimal},
};

use core::ops::{Add, Deref, Mul, MulAssign};

use super::{ConstDecimals, NumDecimals};

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> MulAssign<&ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn mul_assign(&mut self, rhs: &ManagedDecimal<M, D2>) {
        self.data *= &rhs.data;
        self.data /= rhs.scaling_factor().deref();
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> MulAssign<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    #[inline]
    fn mul_assign(&mut self, rhs: ManagedDecimal<M, D2>) {
        self.mul_assign(&rhs);
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> Mul<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
where
    D1: Add<D2>,
    <D1 as Add<D2>>::Output: Decimals,
{
    type Output = ManagedDecimal<M, <D1 as Add<D2>>::Output>;

    fn mul(self, other: ManagedDecimal<M, D2>) -> Self::Output {
        ManagedDecimal {
            data: self.data * other.data,
            decimals: self.decimals + other.decimals,
        }
    }
}

impl<M: ManagedTypeApi, D1: Decimals> ManagedDecimal<M, D1> {
    pub fn mul_with_precision<D2: Decimals, T: Decimals>(
        self,
        other: ManagedDecimal<M, D2>,
        precision: T,
    ) -> ManagedDecimal<M, T> {
        let result = ManagedDecimal {
            data: self.data * other.data,
            decimals: self.decimals.num_decimals() + other.decimals.num_decimals(),
        };
        result.rescale(precision)
    }
}

// var + const
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Mul<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn mul(self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self * rhs.into_var_decimals()
    }
}

// const + var
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Mul<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn mul(self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() * rhs
    }
}
