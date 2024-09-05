use crate::{
    api::ManagedTypeApi,
    types::{ConstDecimals, Decimals, ManagedDecimal, NumDecimals},
};

use core::ops::{Add, AddAssign};

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> AddAssign<&ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn add_assign(&mut self, rhs: &ManagedDecimal<M, D2>) {
        let scaled_data = rhs.rescale_data(self.scale().num_decimals());
        self.data += scaled_data;
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> AddAssign<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    #[inline]
    fn add_assign(&mut self, rhs: ManagedDecimal<M, D2>) {
        self.add_assign(&rhs);
    }
}

// const + const
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Add<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn add(mut self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self.data += rhs.data;
        self
    }
}

// var + var
impl<M: ManagedTypeApi> Add<ManagedDecimal<M, NumDecimals>> for ManagedDecimal<M, NumDecimals> {
    type Output = Self;

    fn add(mut self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        match self.decimals.cmp(&rhs.decimals) {
            core::cmp::Ordering::Less => {
                self = self.rescale(rhs.decimals);
                self.data += rhs.data;
            },
            core::cmp::Ordering::Equal => self.data += rhs.data,
            core::cmp::Ordering::Greater => {
                let rhs_data = rhs.rescale_data(self.decimals);
                self.data += rhs_data;
            },
        }
        self
    }
}

// var + const
impl<const DECIMALS: usize, M: ManagedTypeApi> Add<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn add(self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self + rhs.into_var_decimals()
    }
}

// const + var
impl<const DECIMALS: usize, M: ManagedTypeApi> Add<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn add(self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() + rhs
    }
}
