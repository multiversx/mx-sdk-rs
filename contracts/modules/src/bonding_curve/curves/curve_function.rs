multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::utils::structs::CurveArguments;
pub trait CurveFunction<M: ManagedTypeApi> {
    fn calculate_price(
        &self,
        token_start: &BigUint<M>,
        amount: &BigUint<M>,
        arguments: &CurveArguments<M>,
    ) -> BigUint<M>;
}
