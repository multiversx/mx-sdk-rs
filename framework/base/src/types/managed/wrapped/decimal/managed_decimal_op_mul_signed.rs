use crate::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{Decimals, ManagedDecimalSigned},
};

use core::ops::{Add, Mul, MulAssign};

use super::{ConstDecimals, NumDecimals};

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> MulAssign<&ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    fn mul_assign(&mut self, rhs: &ManagedDecimalSigned<M, D2>) {
        self.data *= &rhs.data;
        self.data /= rhs.scaling_factor().as_big_int();
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> MulAssign<ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    #[inline]
    fn mul_assign(&mut self, rhs: ManagedDecimalSigned<M, D2>) {
        self.mul_assign(&rhs);
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> Mul<ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
where
    D1: Add<D2>,
    <D1 as Add<D2>>::Output: Decimals,
{
    type Output = ManagedDecimalSigned<M, <D1 as Add<D2>>::Output>;

    fn mul(self, other: ManagedDecimalSigned<M, D2>) -> Self::Output {
        ManagedDecimalSigned {
            data: self.data * other.data,
            decimals: self.decimals + other.decimals,
        }
    }
}

impl<M: ManagedTypeApi, D1: Decimals> ManagedDecimalSigned<M, D1> {
    pub fn mul_with_precision<D2: Decimals, T: Decimals>(
        self,
        other: ManagedDecimalSigned<M, D2>,
        precision: T,
    ) -> ManagedDecimalSigned<M, T> {
        let result = ManagedDecimalSigned {
            data: self.data * other.data,
            decimals: self.decimals.num_decimals() + other.decimals.num_decimals(),
        };
        result.rescale(precision)
    }
}

// var + const
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Mul<ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimalSigned<M, NumDecimals>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn mul(self, rhs: ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self * rhs.into_var_decimals()
    }
}

// const + var
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Mul<ManagedDecimalSigned<M, NumDecimals>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn mul(self, rhs: ManagedDecimalSigned<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() * rhs
    }
}
