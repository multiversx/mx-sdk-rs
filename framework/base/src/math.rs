/// Only used internally for computing logarithms for ManagedDecimal and BigUint.
pub(crate) mod internal_logarithm_i64;
mod linear_interpolation;
mod weighted_average;

pub use linear_interpolation::{LinearInterpolationInvalidValuesError, linear_interpolation};
pub use weighted_average::{weighted_average, weighted_average_round_up};
