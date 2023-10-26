multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::{curves::curve_function::CurveFunction, utils::structs::CurveArguments};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct LinearFunction<M: ManagedTypeApi> {
    pub initial_price: BaseBigUint<M>,
    pub linear_coefficient: BaseBigUint<M>,
}

impl<M: ManagedTypeApi> CurveFunction<M> for LinearFunction<M> {
    fn calculate_price(
        &self,
        token_start: &BaseBigUint<M>,
        amount: &BaseBigUint<M>,
        _arguments: &CurveArguments<M>,
    ) -> BaseBigUint<M> {
        &self.linear_coefficient * &sum_interval(amount, token_start) + &self.initial_price * amount
    }
}

fn sum_interval<M: ManagedTypeApi>(n: &BaseBigUint<M>, x: &BaseBigUint<M>) -> BaseBigUint<M> {
    x * n + &(n - 1u32) * n / 2u32
}
