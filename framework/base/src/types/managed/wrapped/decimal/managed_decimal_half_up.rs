use core::ops::Deref;

use crate::{api::ManagedTypeApi, types::Sign};

use super::{Decimals, ManagedDecimal, ManagedDecimalSigned};

impl<M: ManagedTypeApi, D1: Decimals> ManagedDecimal<M, D1> {
    /// Multiplies two decimals with half-up rounding to a target precision.
    ///
    /// Both operands are first rescaled to `precision`. The rescaled raw values
    /// are multiplied, producing a result with `2 * precision` implied decimal
    /// places. That intermediate value is then rounded back to `precision` using
    /// the standard pre-bias trick:
    ///
    /// ```text
    /// rounded = (product + scale / 2) / scale
    /// ```
    ///
    /// Adding `scale / 2` before the integer division means that any remainder
    /// ≥ half the scale (i.e. the fractional part ≥ 0.5) causes the quotient
    /// to increment by one — equivalent to round-half-up.
    ///
    /// # Credits
    /// Original implementation by [@mihaieremia](https://github.com/mihaieremia).
    pub fn mul_half_up<D2: Decimals, DResult: Decimals>(
        &self,
        other: &ManagedDecimal<M, D2>,
        precision: DResult,
    ) -> ManagedDecimal<M, DResult> {
        // Use target precision directly, no +1
        let scaled_a = self.rescale(precision.clone());
        let scaled_b = other.rescale(precision.clone());

        // Perform multiplication in BigUint
        let product = scaled_a.data * scaled_b.data;

        // Half-up rounding at precision
        let scale = precision.scaling_factor();
        let half_scaled = scale.deref().clone() / 2u64;

        // Round half-up
        let rounded_product = (product + half_scaled) / &*scale;

        ManagedDecimal::from_raw_units(rounded_product, precision)
    }

    /// Divides two decimals with half-up rounding to a target precision.
    ///
    /// Both operands are rescaled to `precision`. The numerator is then
    /// multiplied by `scale` so the division produces a result with the correct
    /// number of decimal places. The quotient is rounded using the pre-bias
    /// trick:
    ///
    /// ```text
    /// rounded = (numerator * scale + denominator / 2) / denominator
    /// ```
    ///
    /// Adding `denominator / 2` means that once the true quotient's remainder
    /// reaches half the denominator (i.e. fractional part ≥ 0.5), integer
    /// division increments the result — equivalent to round-half-up.
    ///
    /// # Credits
    /// Original implementation by [@mihaieremia](https://github.com/mihaieremia).
    pub fn div_half_up<D2: Decimals, DResult: Decimals>(
        &self,
        other: &ManagedDecimal<M, D2>,
        precision: DResult,
    ) -> ManagedDecimal<M, DResult> {
        let scaled_a = self.rescale(precision.clone());
        let scaled_b = other.rescale(precision.clone());

        // Perform division in BigUint
        let scale = precision.scaling_factor();
        let numerator = scaled_a.into_raw_units() * &*scale;
        let denominator = scaled_b.into_raw_units();

        // Half-up rounding
        let half_denominator = denominator.clone() / 2u64;
        let rounded_quotient = (numerator + half_denominator) / denominator;

        ManagedDecimal::from_raw_units(rounded_quotient, precision)
    }
}

impl<M: ManagedTypeApi, D1: Decimals> ManagedDecimalSigned<M, D1> {
    /// Multiplies two signed decimals with half-up (away-from-zero) rounding
    /// to a target precision.
    ///
    /// The algorithm mirrors [`ManagedDecimal::mul_half_up`], but uses `BigInt`
    /// arithmetic and adjusts the pre-bias direction based on the sign of the
    /// intermediate product:
    ///
    /// ```text
    /// if product < 0:  rounded = (product - scale / 2) / scale
    /// else:            rounded = (product + scale / 2) / scale
    /// ```
    ///
    /// The VM's integer division truncates toward zero. Subtracting the bias
    /// for a negative product pushes it *further* from zero before truncation,
    /// so the final result rounds away from zero in both directions — matching
    /// the conventional financial definition of "round half up" for signed
    /// numbers.
    ///
    /// # Credits
    /// Original implementation by [@mihaieremia](https://github.com/mihaieremia).
    pub fn mul_half_up_signed<D2: Decimals, DResult: Decimals>(
        &self,
        other: &ManagedDecimalSigned<M, D2>,
        precision: DResult,
    ) -> ManagedDecimalSigned<M, DResult> {
        let scaled_a = self.rescale(precision.clone());
        let scaled_b = other.rescale(precision.clone());

        // Perform multiplication in BigInt
        let product = scaled_a.data * scaled_b.data;

        // Half-up rounding at precision
        let scale = precision.scaling_factor();
        let half_scaled = (scale.deref().clone() / 2u64).into_big_int();

        // Sign-aware "away-from-zero" rounding
        let rounded_product = if product.sign() == Sign::Minus {
            (product - half_scaled) / scale.as_big_int()
        } else {
            (product + half_scaled) / scale.as_big_int()
        };

        ManagedDecimalSigned::from_raw_units(rounded_product, precision)
    }

    /// Divides two signed decimals with half-up (away-from-zero) rounding
    /// to a target precision.
    ///
    /// The numerator is scaled up by `scale` (as in [`ManagedDecimal::div_half_up`])
    /// and then pre-biased before T-division (truncates toward zero). The bias
    /// direction depends solely on the sign of the numerator — **not** on the
    /// sign of the denominator:
    ///
    /// ```text
    /// half = |denominator| / 2
    /// if numerator < 0:  rounded = (numerator - half) / denominator
    /// else:              rounded = (numerator + half) / denominator
    /// ```
    ///
    /// This is correct because `half` is always non-negative. When `numerator > 0`,
    /// adding `half` increases the numerator's magnitude; when the denominator is
    /// negative, dividing a larger positive numerator yields a more-negative
    /// result — farther from zero. The rule therefore rounds away from zero for
    /// all four sign combinations of `(numerator, denominator)`.
    ///
    /// Using `sign(denominator)` as the branch condition instead would produce
    /// wrong results whenever the denominator is negative.
    ///
    /// # Credits
    /// Original implementation by [@mihaieremia](https://github.com/mihaieremia).
    pub fn div_half_up_signed<D2: Decimals, DResult: Decimals>(
        &self,
        other: &ManagedDecimalSigned<M, D2>,
        precision: DResult,
    ) -> ManagedDecimalSigned<M, DResult> {
        let scaled_a = self.rescale(precision.clone());
        let scaled_b = other.rescale(precision.clone());

        let scale = precision.scaling_factor();
        let numerator = scaled_a.data * scale.as_big_int();
        let denominator = scaled_b.data;

        // Half-up rounding
        let half_denominator = (denominator.magnitude() / 2u64).into_big_int();
        let rounded_quotient = if numerator.sign() == Sign::Minus {
            (numerator - half_denominator) / denominator
        } else {
            (numerator + half_denominator) / denominator
        };

        ManagedDecimalSigned::from_raw_units(rounded_quotient, precision)
    }
}
