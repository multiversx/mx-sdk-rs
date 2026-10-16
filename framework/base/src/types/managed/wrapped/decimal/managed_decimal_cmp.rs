use core::cmp::Ordering;

use crate::{
    api::ManagedTypeApi,
    types::{BigUint, Decimals, ManagedDecimal},
};

use super::scaling_factor::scaling_factor;

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> PartialEq<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn eq(&self, other: &ManagedDecimal<M, D2>) -> bool {
        match self
            .decimals
            .num_decimals()
            .cmp(&other.decimals.num_decimals())
        {
            Ordering::Less => {
                let diff_decimals = other.decimals.num_decimals() - self.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &scaling_factor::<M>(diff_decimals);
                &self.data * scaling_factor == other.data
            }
            Ordering::Equal => self.data == other.data,
            Ordering::Greater => {
                let diff_decimals = self.decimals.num_decimals() - other.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &scaling_factor::<M>(diff_decimals);
                &other.data * scaling_factor == self.data
            }
        }
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> PartialOrd<ManagedDecimal<M, D2>>
    for ManagedDecimal<M, D1>
{
    fn partial_cmp(&self, other: &ManagedDecimal<M, D2>) -> Option<Ordering> {
        match self
            .decimals
            .num_decimals()
            .cmp(&other.decimals.num_decimals())
        {
            Ordering::Less => {
                let diff_decimals = other.decimals.num_decimals() - self.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &scaling_factor::<M>(diff_decimals);

                Some((&self.data * scaling_factor).cmp(&other.data))
            }
            Ordering::Equal => Some((self.data).cmp(&other.data)),
            Ordering::Greater => {
                let diff_decimals = self.decimals.num_decimals() - other.decimals.num_decimals();
                let scaling_factor: &BigUint<M> = &scaling_factor::<M>(diff_decimals);
                Some((&other.data * scaling_factor).cmp(&self.data))
            }
        }
    }
}
