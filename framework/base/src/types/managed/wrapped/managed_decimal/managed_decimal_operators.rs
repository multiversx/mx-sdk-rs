use crate::{
    api::ManagedTypeApi,
    types::{BigUint, ConstDecimals, Decimals, ManagedDecimal, NumDecimals},
};

use core::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Add<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn add(self, other: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data + other.data)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> Sub<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = Self;

    fn sub(self, other: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        ManagedDecimal::const_decimals_from_raw(self.data - other.data)
    }
}

impl<M: ManagedTypeApi> Add<ManagedDecimal<M, NumDecimals>> for ManagedDecimal<M, NumDecimals> {
    type Output = Self;

    fn add(self, other: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        let scaled = other.rescale(self.scale());
        ManagedDecimal::from_raw_units(&self.data + &scaled.data, scaled.decimals)
    }
}

impl<M: ManagedTypeApi> Sub<ManagedDecimal<M, NumDecimals>> for ManagedDecimal<M, NumDecimals> {
    type Output = Self;

    fn sub(self, other: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        let scaled = other.rescale(self.scale());
        ManagedDecimal::from_raw_units(&self.data - &scaled.data, scaled.decimals)
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

impl<M: ManagedTypeApi, D1: Decimals, D2: Decimals> MulToPrecision<M, D2>
    for ManagedDecimal<M, D1>
{
    fn mul_with_precision<T: Decimals>(
        self,
        other: ManagedDecimal<M, D2>,
        precision: T,
    ) -> ManagedDecimal<M, T> {
        let scaled;
        let new_data;
        if self.decimals.num_decimals() >= other.decimals.num_decimals() {
            scaled = other.rescale(self.scale());
            new_data = self.data * scaled.data;
        } else {
            scaled = self.rescale(other.scale());
            new_data = scaled.data * other.data;
        }
        ManagedDecimal {
            data: new_data,
            decimals: precision,
        }
    }
}

pub trait MulToPrecision<M: ManagedTypeApi, D: Decimals> {
    fn mul_with_precision<T: Decimals>(
        self,
        other: ManagedDecimal<M, D>,
        precision: T,
    ) -> ManagedDecimal<M, T>;
}

impl<const DECIMALS: usize, M: ManagedTypeApi> Sub<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn sub(self, rhs: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        if DECIMALS >= rhs.decimals {
            let scaled = rhs.rescale(self.scale());
            ManagedDecimal {
                data: self.data - scaled.data,
                decimals: DECIMALS,
            }
        } else {
            let scaled = self.rescale(rhs.scale());
            ManagedDecimal {
                data: scaled.data - rhs.data,
                decimals: rhs.decimals,
            }
        }
    }
}

impl<const DECIMALS: usize, M: ManagedTypeApi> Sub<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = Self;

    fn sub(self, rhs: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        if DECIMALS >= self.decimals {
            let scaled = self.rescale(rhs.scale());
            ManagedDecimal {
                data: self.data - scaled.data,
                decimals: DECIMALS,
            }
        } else {
            let scaled = rhs.rescale(self.scale());
            ManagedDecimal {
                data: scaled.data - rhs.data,
                decimals: self.decimals,
            }
        }
    }
}

impl<const DECIMALS: usize, M: ManagedTypeApi> Add<ManagedDecimal<M, ConstDecimals<DECIMALS>>>
    for ManagedDecimal<M, NumDecimals>
{
    type Output = Self;

    fn add(self, other: ManagedDecimal<M, ConstDecimals<DECIMALS>>) -> Self::Output {
        if DECIMALS >= self.decimals {
            let scaled = self.rescale(other.scale());
            ManagedDecimal {
                data: self.data + scaled.data,
                decimals: DECIMALS,
            }
        } else {
            let scaled = other.rescale(self.scale());
            ManagedDecimal {
                data: scaled.data + other.data,
                decimals: self.decimals,
            }
        }
    }
}

impl<const DECIMALS: usize, M: ManagedTypeApi> Add<ManagedDecimal<M, NumDecimals>>
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    type Output = ManagedDecimal<M, NumDecimals>;

    fn add(self, other: ManagedDecimal<M, NumDecimals>) -> Self::Output {
        if DECIMALS >= other.decimals {
            let scaled = other.rescale(self.scale());
            ManagedDecimal {
                data: self.data + scaled.data,
                decimals: self.decimals.num_decimals(),
            }
        } else {
            let scaled = self.rescale(other.scale());
            ManagedDecimal {
                data: scaled.data + other.data,
                decimals: scaled.decimals,
            }
        }
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

    // maybe rescale to highest first
    fn div(self, other: ManagedDecimal<M, D2>) -> Self::Output {
        ManagedDecimal {
            data: self.data / other.data,
            decimals: self.decimals - other.decimals,
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> SubAssign
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn sub_assign(&mut self, rhs: Self) {
        if DECIMALS >= rhs.decimals.num_decimals() {
            let scaled = rhs.rescale(self.scale());
            self.data -= scaled.data;
        } else {
            let scaled = self.rescale(rhs.scale());
            self.data -= scaled.data;
            self.decimals = rhs.decimals;
        }
    }
}

impl<M: ManagedTypeApi, const DECIMALS: usize> AddAssign
    for ManagedDecimal<M, ConstDecimals<DECIMALS>>
{
    fn add_assign(&mut self, rhs: Self) {
        if DECIMALS >= rhs.decimals.num_decimals() {
            let scaled = rhs.rescale(self.scale());
            self.data += scaled.data;
        } else {
            let scaled = self.rescale(rhs.scale());
            self.data = scaled.data + rhs.data;
            self.decimals = rhs.decimals;
        }
    }
}
