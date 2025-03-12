use super::decimals::{Decimals, LnDecimals};
use super::ManagedDecimalSigned;
use super::{ManagedDecimal, NumDecimals};

use crate::types::ManagedRef;
use crate::{
    api::ManagedTypeApi,
    contract_base::ErrorHelper,
    types::{BigInt, BigUint, Sign},
};

fn compute_ln<M: ManagedTypeApi>(
    data: &BigUint<M>,
    num_decimals: NumDecimals,
) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
    // start with aproximation, based on position of the most significant bit
    let Some(log2_floor) = data.log2_floor() else {
        // means the input was zero
        return None;
    };

    let scaling_factor_9 = LnDecimals::new().scaling_factor();
    let divisor = BigUint::from(1u64) << log2_floor as usize;
    let normalized = data * &*scaling_factor_9 / divisor;

    let x = normalized
        .to_u64()
        .unwrap_or_else(|| ErrorHelper::<M>::signal_error_with_message("ln internal error"))
        as i64;

    let mut result = crate::types::math_util::logarithm_i64::ln_polynomial(x);
    crate::types::math_util::logarithm_i64::ln_add_bit_log2(&mut result, log2_floor);

    debug_assert!(result > 0);

    crate::types::math_util::logarithm_i64::ln_sub_decimals(&mut result, num_decimals);

    Some(ManagedDecimalSigned::from_raw_units(
        BigInt::from(result),
        LnDecimals::new(),
    ))
}

fn compute_log2<M: ManagedTypeApi>(
    data: &BigUint<M>,
    num_decimals: NumDecimals,
) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
    // start with aproximation, based on position of the most significant bit
    let Some(log2_floor) = data.log2_floor() else {
        // means the input was zero
        return None;
    };

    let scaling_factor_9 = LnDecimals::new().scaling_factor();
    let divisor = BigUint::from(1u64) << log2_floor as usize;
    let normalized = data * &*scaling_factor_9 / divisor;

    let x = normalized
        .to_u64()
        .unwrap_or_else(|| ErrorHelper::<M>::signal_error_with_message("log2 internal error"))
        as i64;

    let mut result = crate::types::math_util::logarithm_i64::log2_polynomial(x);
    crate::types::math_util::logarithm_i64::log2_add_bit_log2(&mut result, log2_floor);

    debug_assert!(result > 0);

    crate::types::math_util::logarithm_i64::log2_sub_decimals(&mut result, num_decimals);

    Some(ManagedDecimalSigned::from_raw_units(
        BigInt::from(result),
        LnDecimals::new(),
    ))
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimal<M, D> {
    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn ln(&self) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
        compute_ln(&self.data, self.decimals.num_decimals())
    }

    /// Base 2 logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn log2(&self) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
        compute_log2(&self.data, self.decimals.num_decimals())
    }
}

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimalSigned<M, D> {
    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn ln(&self) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
        if self.sign() != Sign::Plus {
            return None;
        }

        let bu = unsafe { ManagedRef::wrap_handle(self.data.handle.clone()) };
        compute_ln(&bu, self.decimals.num_decimals())
    }

    /// Base 2 logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// Even though 9 decimals are returned, only around 6 decimals are actually useful.
    pub fn log2(&self) -> Option<ManagedDecimalSigned<M, LnDecimals>> {
        if self.sign() != Sign::Plus {
            return None;
        }

        let bu = unsafe { ManagedRef::wrap_handle(self.data.handle.clone()) };
        compute_log2(&bu, self.decimals.num_decimals())
    }
}
