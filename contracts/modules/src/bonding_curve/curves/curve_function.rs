use multiversx_sc::imports::*;

use crate::bonding_curve::utils::structs::CurveArguments;
pub trait CurveFunction<M: ManagedTypeApi> {
    fn calculate_price(
        &self,
        token_start: &BigUint<M>,
        amount: &BigUint<M>,
        arguments: &CurveArguments<M>,
    ) -> BigUint<M>;
}
