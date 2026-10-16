use core::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use typenum::{U9, U18, Unsigned};

/// Decimals are represented as usize. This type is also used as variable decimals.
pub type NumDecimals = usize;

/// Implemented by all decimal types usable in `ManagedDecimal`.
pub trait Decimals: Clone {
    /// Number of decimals as variable.
    fn num_decimals(&self) -> NumDecimals;
}

impl Decimals for NumDecimals {
    fn num_decimals(&self) -> NumDecimals {
        *self
    }
}

/// Zero-sized constant number of decimals.
///
/// Ideal if the number of decimals is known at compile time.
#[derive(Clone, Default, Debug)]
pub struct ConstDecimals<DECIMALS: Unsigned> {
    _phantom: PhantomData<DECIMALS>,
}

/// Alias of the const decimal type that we use to compute the logarithm.
///
/// We always compute it with 9 decimals.
pub type LnDecimals = ConstDecimals<U9>;

/// Alias of the type that represents the number of decimal of the EGLD, which is 18.
pub type EgldDecimals = ConstDecimals<U18>;

impl<DECIMALS: Unsigned> ConstDecimals<DECIMALS> {
    pub const fn new() -> Self {
        ConstDecimals {
            _phantom: PhantomData,
        }
    }
}

impl<DECIMALS: Unsigned> Decimals for ConstDecimals<DECIMALS> {
    fn num_decimals(&self) -> NumDecimals {
        DECIMALS::to_usize()
    }
}

impl<DEC1, DEC2> Add<ConstDecimals<DEC2>> for ConstDecimals<DEC1>
where
    DEC1: Unsigned,
    DEC2: Unsigned,
    DEC1: Add<DEC2>,
    <DEC1 as Add<DEC2>>::Output: Unsigned,
{
    type Output = ConstDecimals<<DEC1 as Add<DEC2>>::Output>;
    fn add(self, _rhs: ConstDecimals<DEC2>) -> Self::Output {
        ConstDecimals::new()
    }
}

impl<DEC1, DEC2> Sub<ConstDecimals<DEC2>> for ConstDecimals<DEC1>
where
    DEC1: Unsigned,
    DEC2: Unsigned,
    DEC1: Sub<DEC2>,
    <DEC1 as Sub<DEC2>>::Output: Unsigned,
{
    type Output = ConstDecimals<<DEC1 as Sub<DEC2>>::Output>;
    fn sub(self, _rhs: ConstDecimals<DEC2>) -> Self::Output {
        ConstDecimals::new()
    }
}

// Mixed const/variable combinations always resolve to a variable number of decimals,
// since the number of decimals can no longer be known at compile time.
impl<DECIMALS: Unsigned> Add<ConstDecimals<DECIMALS>> for NumDecimals {
    type Output = NumDecimals;
    fn add(self, rhs: ConstDecimals<DECIMALS>) -> Self::Output {
        self + rhs.num_decimals()
    }
}

impl<DECIMALS: Unsigned> Add<NumDecimals> for ConstDecimals<DECIMALS> {
    type Output = NumDecimals;
    fn add(self, rhs: NumDecimals) -> Self::Output {
        self.num_decimals() + rhs
    }
}

impl<DECIMALS: Unsigned> Sub<ConstDecimals<DECIMALS>> for NumDecimals {
    type Output = NumDecimals;
    fn sub(self, rhs: ConstDecimals<DECIMALS>) -> Self::Output {
        self - rhs.num_decimals()
    }
}

impl<DECIMALS: Unsigned> Sub<NumDecimals> for ConstDecimals<DECIMALS> {
    type Output = NumDecimals;
    fn sub(self, rhs: NumDecimals) -> Self::Output {
        self.num_decimals() - rhs
    }
}
