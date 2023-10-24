multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::utils::structs::CurveArguments;
pub trait CurveFunction<M: ManagedTypeApi> {
    fn calculate_price(
        &self,
        token_start: &BaseBigUint<M>,
        amount: &BaseBigUint<M>,
        arguments: &CurveArguments<M>,
    ) -> BaseBigUint<M>;
}
