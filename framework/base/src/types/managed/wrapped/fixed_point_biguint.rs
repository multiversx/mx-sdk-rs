use crate::{api::ManagedTypeApi, types::BigUint};

use core::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct FixedPoint<M: ManagedTypeApi, const DECIMALS: usize> {
    data: BigUint<M>,
}

impl<M: ManagedTypeApi, const DECIMALS: usize> FixedPoint<M, DECIMALS> {
    pub fn scaling_factor() -> BigUint<M> {
        // TODO: cache
        BigUint::from(10u32).pow(DECIMALS as u32)
    }

    pub fn trunc(&self) -> BigUint<M> {
        &self.data / &Self::scaling_factor()
    }

    pub fn data(&self) -> &BigUint<M> {
        &self.data
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> From<BigUint<M>> for FixedPoint<M, DECIMALS> {
    fn from(value: BigUint<M>) -> Self {
        FixedPoint {
            data: value * Self::scaling_factor(),
        }
    }
}

// impl<M: ManagedTypeApi, const DECIMALS: usize, const OTHER_DECIMALS: usize>
//     From<FixedPoint<M, OTHER_DECIMALS>> for FixedPoint<M, DECIMALS>
// {
//     fn from(self) -> FixedPoint<M, OTHER_DECIMALS> {
//         FixedPoint::<M, OTHER_DECIMALS>::from(
//             self.data / FixedPoint::<M, DECIMALS>::scaling_factor()
//                 * FixedPoint::<M, OTHER_DECIMALS>::scaling_factor(),
//         )
//     }
// }

pub trait Convert<M: ManagedTypeApi, const OTHER_DECIMALS: usize, const DECIMALS: usize> {
    fn convert_from(other: FixedPoint<M, OTHER_DECIMALS>) -> FixedPoint<M, DECIMALS>;
}
impl<M: ManagedTypeApi, const DECIMALS: usize, const OTHER_DECIMALS: usize>
    Convert<M, OTHER_DECIMALS, DECIMALS> for FixedPoint<M, DECIMALS>
{
    fn convert_from(other: FixedPoint<M, OTHER_DECIMALS>) -> FixedPoint<M, DECIMALS> {
        FixedPoint::<M, DECIMALS>::from(
            other.data / FixedPoint::<M, OTHER_DECIMALS>::scaling_factor(),
        )
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> Add<FixedPoint<M, DECIMALS>>
    for FixedPoint<M, DECIMALS>
{
    type Output = Self;

    fn add(self, other: FixedPoint<M, DECIMALS>) -> Self::Output {
        FixedPoint::<M, DECIMALS>::from((self.data + other.data) / &Self::scaling_factor())
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> Sub<FixedPoint<M, DECIMALS>>
    for FixedPoint<M, DECIMALS>
{
    type Output = Self;

    fn sub(self, other: FixedPoint<M, DECIMALS>) -> Self::Output {
        FixedPoint::<M, DECIMALS>::from((self.data - other.data) / &Self::scaling_factor())
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize, const OTHER_DECIMALS: usize>
    Mul<FixedPoint<M, OTHER_DECIMALS>> for FixedPoint<M, DECIMALS>
where
    [(); DECIMALS + OTHER_DECIMALS]:,
{
    type Output = FixedPoint<M, { DECIMALS + OTHER_DECIMALS }>;

    fn mul(self, other: FixedPoint<M, OTHER_DECIMALS>) -> Self::Output {
        FixedPoint::<M, { DECIMALS + OTHER_DECIMALS }>::from(
            self.data * other.data
                / FixedPoint::<M, { DECIMALS + OTHER_DECIMALS }>::scaling_factor(),
        )
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize, const OTHER_DECIMALS: usize>
    Div<FixedPoint<M, OTHER_DECIMALS>> for FixedPoint<M, DECIMALS>
where
    [(); DECIMALS - OTHER_DECIMALS]:,
{
    type Output = FixedPoint<M, { DECIMALS - OTHER_DECIMALS }>;

    fn div(self, other: FixedPoint<M, OTHER_DECIMALS>) -> Self::Output {
        FixedPoint::<M, { DECIMALS - OTHER_DECIMALS }>::from(
            self.data
                / other.data
                / FixedPoint::<M, { DECIMALS - OTHER_DECIMALS }>::scaling_factor(),
        )
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> PartialEq<FixedPoint<M, DECIMALS>>
    for FixedPoint<M, DECIMALS>
{
    fn eq(&self, other: &FixedPoint<M, DECIMALS>) -> bool {
        self.data == other.data
    }
}
