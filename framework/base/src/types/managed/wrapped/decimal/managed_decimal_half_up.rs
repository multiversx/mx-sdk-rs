use crate::{api::ManagedTypeApi, types::Sign};

use super::{Decimals, ManagedDecimal, ManagedDecimalSigned};

impl<M: ManagedTypeApi, D1: Decimals> ManagedDecimal<M, D1> {
    /// Multiplies two decimals with half-up rounding at target precision.
    /// Prevents precision loss in financial calculations using half-up rounding.
    /// Returns product rounded to specified precision.
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
        let half_scaled = &*scale / 2u64;

        // Round half-up
        let rounded_product = (product + half_scaled) / &*scale;

        ManagedDecimal::from_raw_units(rounded_product, precision)
    }

    /// Divides two decimals with half-up rounding at target precision.
    /// Prevents precision loss in financial calculations using half-up rounding.
    /// Returns quotient rounded to specified precision.
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
    /// Multiplies two signed decimals with half-up rounding away from zero.
    /// Handles negative values correctly for financial calculations.
    /// Returns signed product rounded to specified precision.
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
        let half_scaled = (scale.clone() / 2u64).into_big_int();

        // Sign-aware "away-from-zero" rounding
        let rounded_product = if product.sign() == Sign::Minus {
            (product - half_scaled) / scale.as_big_int()
        } else {
            (product + half_scaled) / scale.as_big_int()
        };

        ManagedDecimalSigned::from_raw_units(rounded_product, precision)
    }

    /// Divides two signed decimals with half-up rounding away from zero.
    /// Handles negative values correctly for financial calculations.
    /// Returns signed quotient rounded to specified precision.
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
