use crate::{
    api::ManagedTypeApi,
    types::{ConstDecimals, Decimals, ManagedDecimal, NumDecimals},
};

use core::ops::{Sub, SubAssign};

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> SubAssign<&ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn sub_assign(&mut self, rhs: &ManagedDecimal<M, D2>) {
        let scaled_data = rhs.rescale_data(self.scale().num_decimals());
        self.data -= scaled_data;
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> SubAssign<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    #[inline]
    fn sub_assign(&mut self, rhs: ManagedDecimal<M, D2>) {
        self.sub_assign(&rhs);
    }
}

// const + const
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Sub<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn sub(mut self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self.data -= rhs.data;
        self
    }
}

// var + var
impl<M: ManagedTypeApi> Sub<ManagedDecimal<M, NumDecimals>> for ManagedDecimal<M, NumDecimals> {
    type Output = Self;

    fn sub(mut self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        match self.decimals.cmp(&rhs.decimals) {
            core::cmp::Ordering::Less => {
                self = self.rescale(rhs.decimals);
                self.data -= rhs.data;
            },
            core::cmp::Ordering::Equal => self.data -= rhs.data,
            core::cmp::Ordering::Greater => {
                let rhs_data = rhs.rescale_data(self.decimals);
                self.data -= rhs_data;
            },
        }
        self
    }
}

// var + const
impl<const DECIMALS: usize, M: ManagedTypeApi> Sub<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn sub(self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self - rhs.into_var_decimals()
    }
}

// const + var
impl<const DECIMALS: usize, M: ManagedTypeApi> Sub<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn sub(self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() - rhs
    }
}
