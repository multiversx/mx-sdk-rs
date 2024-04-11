multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::utils::structs::CurveArguments;
pub trait CurveFunction<'a, M: ManagedTypeApi<'a>> {
    fn calculate_price(
        &self,
        token_start: &BigUint<'a, M>,
        amount: &BigUint<'a, M>,
        arguments: &CurveArguments<'a, M>,
    ) -> BigUint<'a, M>;
}
