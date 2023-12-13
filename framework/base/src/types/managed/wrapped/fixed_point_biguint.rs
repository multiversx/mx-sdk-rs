use crate::{api::ManagedTypeApi, types::BigUint};

use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};

fn scaling_factor<M: ManagedTypeApi>(num_decimals: NumDecimals) -> BigUint<M> {
    // TODO: cache
    BigUint::from(10u32).pow(num_decimals as u32)
}

pub trait Decimals {
    fn num_decimals(&self) -> NumDecimals;

    fn scaling_factor<M: ManagedTypeApi>(&self) -> BigUint<M> {
        scaling_factor(self.num_decimals())
    }
}

impl Decimals for NumDecimals {
    fn num_decimals(&self) -> NumDecimals {
        *self
    }
}

pub type NumDecimals = usize;

#[derive(Clone, Debug)]
pub struct ConstDecimals<const DECIMALS: NumDecimals>;

impl<const DECIMALS: NumDecimals> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS
    }

    fn scaling_factor<M: ManagedTypeApi>(&self) -> BigUint<M> {
        scaling_factor(self.num_decimals())
    }
}

#[derive(Debug, Clone)]
pub struct FixedPoint<M: ManagedTypeApi, D: Decimals> {
    data: BigUint<M>,
    decimals: D,
}

impl<M: ManagedTypeApi, D: Decimals> FixedPoint<M, D> {
    pub fn trunc(&self) -> BigUint<M> {
        &self.data / &self.decimals.scaling_factor()
    }

    pub fn into_raw_units(&self) -> &BigUint<M> {
        &self.data
    }

    pub fn from_raw_units(data: BigUint<M>, decimals: D) -> Self {
        FixedPoint { data, decimals }
    }

    pub fn scale(&self) -> usize {
        self.decimals.num_decimals()
    }

    pub fn rescale<T: Decimals>(self, scale_to: T) -> FixedPoint<M, T>
    where
        M: ManagedTypeApi,
    {
        let from_num_decimals = self.decimals.num_decimals();
        let scale_to_num_decimals = scale_to.num_decimals();

        match from_num_decimals.cmp(&scale_to_num_decimals) {
            Ordering::Less => {
                let delta_decimals = scale_to_num_decimals - from_num_decimals;
                FixedPoint::from_raw_units(&self.data * &scaling_factor(delta_decimals), scale_to)
            },
            Ordering::Equal => FixedPoint::from_raw_units(self.data, scale_to),
            Ordering::Greater => {
                let delta_decimals = from_num_decimals - scale_to_num_decimals;
                FixedPoint::from_raw_units(&self.data * &scaling_factor(delta_decimals), scale_to)
            },
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> FixedPoint<M, ConstDecimals<DECIMALS>> {
    pub fn const_decimals_from_raw(data: BigUint<M>) -> Self {
        FixedPoint {
            data,
            decimals: ConstDecimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<BigUint<M>>
    for FixedPoint<M, ConstDecimals<DECIMALS>>
{
    fn from(value: BigUint<M>) -> Self {
        let decimals = ConstDecimals;
        FixedPoint {
            data: &value * &decimals.scaling_factor(),
            decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Add<FixedPoint<M, ConstDecimals<DECIMALS>>>
    for FixedPoint<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn add(self, other: FixedPoint<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        FixedPoint::const_decimals_from_raw(self.data + other.data)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Sub<FixedPoint<M, ConstDecimals<DECIMALS>>>
    for FixedPoint<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn sub(self, other: FixedPoint<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        FixedPoint::const_decimals_from_raw(self.data - other.data)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const OTHER_DECIMALS: NumDecimals>
    Mul<FixedPoint<M, ConstDecimals<OTHER_DECIMALS>>> for FixedPoint<M, ConstDecimals<DECIMALS>>
where
    [(); DECIMALS + OTHER_DECIMALS]:,
{
    type Output = FixedPoint<M, ConstDecimals<{ DECIMALS + OTHER_DECIMALS }>>;

    fn mul(self, other: FixedPoint<M, ConstDecimals<OTHER_DECIMALS>>) -> Self::Output {
        FixedPoint::const_decimals_from_raw(self.data * other.data)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const OTHER_DECIMALS: NumDecimals>
    Div<FixedPoint<M, ConstDecimals<OTHER_DECIMALS>>> for FixedPoint<M, ConstDecimals<DECIMALS>>
where
    [(); DECIMALS - OTHER_DECIMALS]:,
{
    type Output = FixedPoint<M, ConstDecimals<{ DECIMALS - OTHER_DECIMALS }>>;

    fn div(self, other: FixedPoint<M, ConstDecimals<OTHER_DECIMALS>>) -> Self::Output {
        FixedPoint::const_decimals_from_raw(self.data / other.data)
    }
}

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> PartialEq<FixedPoint<M, D2>>
    for FixedPoint<M, D1>
{
    fn eq(&self, other: &FixedPoint<M, D2>) -> bool {
        match self
            .decimals
            .num_decimals()
            .cmp(&other.decimals.num_decimals())
        {
            Ordering::Less => {
                let diff_decimals = other.decimals.num_decimals() - self.decimals.num_decimals();
                &self.data * &scaling_factor(diff_decimals) == other.data
            },
            Ordering::Equal => self.data == other.data,
            Ordering::Greater => {
                let diff_decimals = self.decimals.num_decimals() - other.decimals.num_decimals();
                &other.data * &scaling_factor(diff_decimals) == self.data
            },
        }
    }
}
