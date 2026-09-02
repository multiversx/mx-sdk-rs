/// Performs subtraction that saturates at zero instead of underflowing, returning a new value.
///
/// # Motivation
///
/// This trait is provided as a custom implementation to support an `Rhs` (Right-Hand Side)
/// generic parameter, mirroring the design of standard library operator traits like
/// `std::ops::Sub` and `std::ops::Add`.
///
/// While the `num` crate provides `num::traits::SaturatingSub`, its signature is strictly
/// bound to references of the exact same type (`fn saturating_sub(&self, v: &Self) -> Self`).
/// Because the Rust standard library lacks a native trait for saturating operators, this
/// trait fills the gap by allowing:
/// * **Mixed-type operations** (e.g., subtracting a `u64` from a `BigUint`).
/// * **Owned-value consumption** (passing by value instead of forcing references).
/// * **Custom `Output` types**.
///
/// For in-place saturating subtraction, see [`SaturatingSubAssign`](super::SaturatingSubAssign).
///
/// # Usage
///
/// Currently implemented for `BigUint`.
///
/// # Examples
///
/// Saturating subtraction clamped to zero on underflow:
///
/// ```ignore
/// let a = BigUint::from(10u32);
/// let b = BigUint::from(15u32);
///
/// assert_eq!(a.saturating_sub(b), BigUint::zero());
/// ```
///
/// Saturating subtraction with a `u64` right-hand side:
///
/// ```ignore
/// let a = BigUint::from(10u32);
///
/// assert_eq!(a.saturating_sub(5u64), BigUint::from(5u32));
/// ```
pub trait SaturatingSub<Rhs = Self> {
    /// The resulting type after applying the saturating subtraction.
    type Output;

    /// Performs the saturating subtraction.
    ///
    /// Computes `self - other`, returning 0 if the result would be negative.
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}
