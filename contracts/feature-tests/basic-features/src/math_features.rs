use multiversx_sc::imports::*;
use multiversx_sc::math;

#[multiversx_sc::module]
pub trait MathFeatures {
    #[endpoint]
    fn math_weighted_average(
        &self,
        first_value: BigUint,
        first_weight: BigUint,
        second_value: BigUint,
        second_weight: BigUint,
    ) -> BigUint {
        math::weighted_average(first_value, first_weight, second_value, second_weight)
    }

    #[endpoint]
    fn math_weighted_average_round_up(
        &self,
        first_value: BigUint,
        first_weight: BigUint,
        second_value: BigUint,
        second_weight: BigUint,
    ) -> BigUint {
        math::weighted_average_round_up(first_value, first_weight, second_value, second_weight)
    }

    #[endpoint]
    fn math_linear_interpolation(
        &self,
        min_in: BigUint,
        max_in: BigUint,
        current_in: BigUint,
        min_out: BigUint,
        max_out: BigUint,
    ) -> BigUint {
        math::linear_interpolation(min_in, max_in, current_in, min_out, max_out)
            .unwrap_or_else(|_| sc_panic!("current_in out of [min_in, max_in] range"))
    }
}
