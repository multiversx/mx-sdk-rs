use crate::{
    api::ManagedTypeApi,
    typenum::Unsigned,
    types::{ConstDecimals, Decimals, ManagedDecimalSigned, NumDecimals},
};

use core::ops::{Add, AddAssign};

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> AddAssign<&ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    fn add_assign(&mut self, rhs: &ManagedDecimalSigned<M, D2>) {
        let scaled_data = rhs.rescale_data(self.scale().num_decimals());
        self.data += scaled_data;
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> AddAssign<ManagedDecimalSigned<M, D2>>
    for ManagedDecimalSigned<M, D1>
{
    #[inline]
    fn add_assign(&mut self, rhs: ManagedDecimalSigned<M, D2>) {
        self.add_assign(&rhs);
    }
}

// const + const
impl<M: ManagedTypeApi, DECIMALS: Unsigned> Add<ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn add(mut self, rhs: ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self.data += rhs.data;
        self
    }
}

// var + var
impl<M: ManagedTypeApi> Add<ManagedDecimalSigned<M, NumDecimals>>
    for ManagedDecimalSigned<M, NumDecimals>
{
    type Output = Self;

    fn add(mut self, rhs: ManagedDecimalSigned<M, NumDecimals>) -> Self::Output {
        match self.decimals.cmp(&rhs.decimals) {
            core::cmp::Ordering::Less => {
                self = self.rescale(rhs.decimals);
                self.data += rhs.data;
            }
            core::cmp::Ordering::Equal => self.data += rhs.data,
            core::cmp::Ordering::Greater => {
                let rhs_data = rhs.rescale_data(self.decimals);
                self.data += rhs_data;
            }
        }
        self
    }
}

// var + const
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Add<ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimalSigned<M, NumDecimals>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn add(self, rhs: ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        self + rhs.into_var_decimals()
    }
}

// const + var
impl<DECIMALS: Unsigned, M: ManagedTypeApi> Add<ManagedDecimalSigned<M, NumDecimals>>
    for ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimalSigned<M, NumDecimals>;

    fn add(self, rhs: ManagedDecimalSigned<M, NumDecimals>) -> Self::Output {
        self.into_var_decimals() + rhs
    }
}
