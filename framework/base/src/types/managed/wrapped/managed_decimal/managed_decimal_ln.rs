pub use super::decimals::{ConstDecimals, Decimals, NumDecimals};
pub use super::managed_decimal_signed::ManagedDecimalSigned;
use super::ManagedDecimal;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{const_handles, use_raw_handle, BigFloatApiImpl, ManagedTypeApi},
    contract_base::ErrorHelper,
    formatter::{FormatBuffer, FormatByteReceiver, SCDisplay},
    types::{BigFloat, BigUint},
};

impl<M: ManagedTypeApi, D: Decimals> ManagedDecimal<M, D> {
    /// Natural logarithm of a number.
    ///
    /// Returns `None` for 0.
    ///
    /// TODO: TEMP impl.
    pub fn ln(&self) -> Option<i64> {
        let bit_log2 = self.data.log2(); // aproximate, based on position of the most significant bit
        if bit_log2 == u32::MAX {
            // means the input was zero, TODO: change log2 return type
            return None;
        }

        let scaling_factor_9 = ConstDecimals::<9>.scaling_factor();
        let divisor = BigUint::from(1u64) << bit_log2 as usize;
        let normalized = &self.data * &*scaling_factor_9 / divisor;

        let x = normalized
            .to_u64()
            .unwrap_or_else(|| ErrorHelper::<M>::signal_error_with_message("ln internal error"))
            as i64;

        let mut result = crate::types::math_util::logarithm_i64::ln_polynomial(x);
        crate::types::math_util::logarithm_i64::ln_add_bit_log2(&mut result, bit_log2);

        debug_assert!(result > 0);

        crate::types::math_util::logarithm_i64::ln_sub_decimals(
            &mut result,
            self.decimals.num_decimals(),
        );

        Some(result)
    }
}

impl<M: ManagedTypeApi, const DECIMALS: NumDecimals> ManagedDecimal<M, ConstDecimals<DECIMALS>> {
    // pub fn log(
    //     &self,
    //     target_base: &ManagedDecimal<M, D>,
    //     precision: D,
    // ) -> ManagedDecimal<M, NumDecimals> {
    //     let num_decimals = precision.num_decimals();
    //     // should verify >= 1
    //     let one = ManagedDecimal::from_raw_units(BigUint::from(1u64), 0usize);
    //     one.rescale(self.scale());
    //     assert!(self >= &one, "wrong input for self");
    //     one.rescale(target_base.scale());
    //     assert!(target_base >= &one, "wrong input for target base");

    //     self.ln(&precision)
    //         * ManagedDecimal::from_raw_units(BigUint::from(num_decimals), num_decimals)
    //         / target_base.ln(&precision)
    //     //this should be done with precision
    // }

    pub fn ln_temp<const PREC: usize>(
        self,
        precision: ConstDecimals<PREC>,
    ) -> ManagedDecimal<M, ConstDecimals<PREC>> {
        let num_decimals = self.decimals.num_decimals() as u32;
        // find the highest power of 2 less than or equal to self
        let log2 = self.data.log2() - num_decimals * BigUint::<M>::from(10u64).log2(); // most significant bit for the actual number
        let divisor = 1 << log2;
        let divisor_scaled =
            BigUint::<M>::from(divisor as u64) * self.decimals.scaling_factor().clone_value();
        let _normalized = self.data / divisor_scaled; // normalize to [1.0, 2.0]
        let x_dec = ManagedDecimal::<M, ConstDecimals<0>>::const_decimals_from_raw(_normalized);
        let x = x_dec.rescale(precision.clone());

        // approximating polynom to get the result
        let mut result = ManagedDecimal::<M, ConstDecimals<9>>::const_decimals_from_raw(
            BigUint::from(56570851u64), // 0.056570851, 9 decimals˝
        )
        .mul_with_precision(x.clone(), precision.clone());
        result = ManagedDecimal::<M, ConstDecimals<8>>::const_decimals_from_raw(BigUint::from(
            44717955u64, // 0.44717955, 8 decimals
        ))
        .rescale(precision.clone())
            - result;
        result = result.mul_with_precision(x.clone(), precision.clone());
        result -= ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            14699568u64, // 1.4699568, 7 decimals
        ))
        .rescale(precision.clone());
        result = result.mul_with_precision(x.clone(), precision.clone());
        result += ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            28212026u64, // 2.8212026, 7 decimals
        ))
        .rescale(precision.clone());
        result = result.mul_with_precision(x.clone(), precision.clone());
        result -= ManagedDecimal::<M, ConstDecimals<7>>::const_decimals_from_raw(BigUint::from(
            17417939u64, // 1.7417939, 7 decimals
        ))
        .rescale(precision.clone());

        let log_2 =
            ManagedDecimal::<M, ConstDecimals<0>>::const_decimals_from_raw(BigUint::from(log2));
        let ln_of_2 = ManagedDecimal::<M, ConstDecimals<8>>::const_decimals_from_raw(
            BigUint::from(69314718u64),
        ); // 0.69314718 8 decimals

        result + log_2.mul_with_precision(ln_of_2, precision.clone())
    }
}
