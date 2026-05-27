/// Performs in-place subtraction that saturates at zero instead of underflowing.
///
/// # Motivation
///
/// This trait complements [`SaturatingSub`](super::SaturatingSub) with an assign variant,
/// mirroring the relationship between `std::ops::Sub` and `std::ops::SubAssign`.
/// It allows subtracting a value in place while clamping the result to zero on underflow,
/// which is the expected behavior for unsigned types such as `BigUint`.
///
/// # Examples
///
/// Saturating subtraction in place, clamped to zero on underflow:
///
/// ```ignore
/// let mut a = BigUint::from(10u32);
/// let b = BigUint::from(15u32);
///
/// a.saturating_sub_assign(&b);
/// assert_eq!(a, BigUint::zero());
/// ```
///
/// Saturating subtraction in place with a `u64` right-hand side:
///
/// ```ignore
/// let mut a = BigUint::from(10u32);
///
/// a.saturating_sub_assign(5u64);
/// assert_eq!(a, BigUint::from(5u32));
/// ```
pub trait SaturatingSubAssign<Rhs = Self> {
    /// Performs the saturating subtraction in place.
    ///
    /// Computes `self - other`, returning 0 if the result would be negative.
    fn saturating_sub_assign(&mut self, rhs: Rhs);
}
