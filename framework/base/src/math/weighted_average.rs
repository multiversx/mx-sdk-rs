use core::ops::{Add, Div, Mul, Sub};

/// Computes the weighted average of two values.
///
/// Returns `(first_value * first_weight + second_value * second_weight) / (first_weight + second_weight)`.
///
/// # Panics
///
/// Panics on division by zero if both weights are zero.
pub fn weighted_average<T>(first_value: T, first_weight: T, second_value: T, second_weight: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Clone,
{
    let weight_sum = first_weight.clone() + second_weight.clone();
    let weighted_sum = first_value * first_weight + second_value * second_weight;
    weighted_sum / weight_sum
}

/// Computes the weighted average of two values, rounded up (ceiling division).
///
/// Equivalent to [`weighted_average`], but rounds the result up instead of truncating:
/// `(weighted_sum + weight_sum - 1) / weight_sum`.
///
/// # Panics
///
/// Panics on division by zero if both weights are zero.
pub fn weighted_average_round_up<T>(
    first_value: T,
    first_weight: T,
    second_value: T,
    second_weight: T,
) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Clone + From<u32>,
{
    let weight_sum = first_weight.clone() + second_weight.clone();
    let weighted_sum = first_value * first_weight + second_value * second_weight;
    (weighted_sum + weight_sum.clone() - T::from(1u32)) / weight_sum
}
