use core::ops::{Add, Div, Mul, Sub};

/// Error returned when `current_in` is outside the `[min_in, max_in]` range.
#[derive(Debug)]
pub struct LinearInterpolationInvalidValuesError;

/// Computes a linearly interpolated output value for a given input within a known range.
///
/// Given an input range `[min_in, max_in]` and a corresponding output range `[min_out, max_out]`,
/// maps `current_in` proportionally to its position in the output range.
///
/// Formula:
/// ```text
/// out = (min_out * (max_in - current_in) + max_out * (current_in - min_in)) / (max_in - min_in)
/// ```
///
/// Returns [`Err(LinearInterpolationInvalidValuesError)`] if `current_in` is outside `[min_in, max_in]`.
///
/// See also: <https://en.wikipedia.org/wiki/Linear_interpolation>
pub fn linear_interpolation<T>(
    min_in: T,
    max_in: T,
    current_in: T,
    min_out: T,
    max_out: T,
) -> Result<T, LinearInterpolationInvalidValuesError>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd + Clone,
{
    if min_in > max_in || current_in < min_in || current_in > max_in {
        return Err(LinearInterpolationInvalidValuesError);
    }

    let min_out_weighted = min_out * (max_in.clone() - current_in.clone());
    let max_out_weighted = max_out * (current_in - min_in.clone());
    let in_diff = max_in - min_in;

    let result = (min_out_weighted + max_out_weighted) / in_diff;
    Ok(result)
}
