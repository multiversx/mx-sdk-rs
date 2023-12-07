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

pub trait DecimalsNeq<D: Decimals> {}

impl<const DECIMALS: NumDecimals> !DecimalsNeq<ConstDecimals<DECIMALS>>
    for ConstDecimals<DECIMALS>
{
}

impl Decimals for NumDecimals {
    fn num_decimals(&self) -> NumDecimals {
        *self
    }
}

pub type NumDecimals = usize;

pub struct ConstDecimals<const DECIMALS: NumDecimals>;

impl<const DECIMALS: NumDecimals> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS
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

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const FROM_DECIMALS: NumDecimals>
    From<FixedPoint<M, ConstDecimals<FROM_DECIMALS>>> for FixedPoint<M, ConstDecimals<DECIMALS>>
where
    ConstDecimals<FROM_DECIMALS>: DecimalsNeq<ConstDecimals<DECIMALS>>,
{
    fn from(
        value: FixedPoint<M, ConstDecimals<FROM_DECIMALS>>,
    ) -> FixedPoint<M, ConstDecimals<DECIMALS>> {
        match DECIMALS.cmp(&FROM_DECIMALS) {
            Ordering::Less => {
                todo!()
                // let delta_scaling_factor = scaling_factor(DECIMALS) - scaling_factor(OTHER_DECIMALS);
                // FixedPoint::const_decimals_from_raw(self.data * delta_scaling_factor)
            },
            Ordering::Equal => FixedPoint::const_decimals_from_raw(value.data),
            Ordering::Greater => {
                todo!()
                // let delta_scaling_factor = scaling_factor(DECIMALS) - scaling_factor(OTHER_DECIMALS);
                // FixedPoint::const_decimals_from_raw(self.data * delta_scaling_factor)
            },
        }
    }
}

impl<M: ManagedTypeApi, const FROM_DECIMALS: NumDecimals>
    From<FixedPoint<M, ConstDecimals<FROM_DECIMALS>>> for FixedPoint<M, NumDecimals>
{
    fn from(value: FixedPoint<M, ConstDecimals<FROM_DECIMALS>>) -> Self {
        FixedPoint {
            data: value.data,
            decimals: FROM_DECIMALS,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> From<FixedPoint<M, NumDecimals>>
    for FixedPoint<M, ConstDecimals<DECIMALS>>
{
    fn from(value: FixedPoint<M, NumDecimals>) -> Self {
        match DECIMALS.cmp(&value.decimals) {
            Ordering::Less => {
                todo!()
                // let delta_scaling_factor = scaling_factor(DECIMALS) - scaling_factor(OTHER_DECIMALS);
                // FixedPoint::const_decimals_from_raw(self.data * delta_scaling_factor)
            },
            Ordering::Equal => unreachable!(),
            Ordering::Greater => {
                todo!()
                // let delta_scaling_factor = scaling_factor(DECIMALS) - scaling_factor(OTHER_DECIMALS);
                // FixedPoint::const_decimals_from_raw(self.data * delta_scaling_factor)
            },
        }
    }
}

// pub trait Convert<M: ManagedTypeApi, const OTHER_DECIMALS: NumDecimals, const DECIMALS: NumDecimals> {
//     fn convert_from(other: FixedPoint<M, OTHER_DECIMALS>) -> FixedPoint<M, DECIMALS>;
// }
// impl<M: ManagedTypeApi, const DECIMALS: NumDecimals, const OTHER_DECIMALS: NumDecimals>
//     Convert<M, OTHER_DECIMALS, DECIMALS> for FixedPoint<M, DECIMALS>
// {
//     fn convert_from(other: FixedPoint<M, OTHER_DECIMALS>) -> FixedPoint<M, DECIMALS> {
//         FixedPoint::<M, DECIMALS>::from_raw_units(
//             other.data / FixedPoint::<M, OTHER_DECIMALS>::scaling_factor()
//                 * FixedPoint::<M, DECIMALS>::scaling_factor(),
//         )
//     }
// }

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
            Ordering::Less => todo!(),
            Ordering::Equal => self.data == other.data,
            Ordering::Greater => todo!(),
        }
    }
}
