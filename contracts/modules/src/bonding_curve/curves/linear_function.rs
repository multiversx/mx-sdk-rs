multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::{curves::curve_function::CurveFunction, utils::structs::CurveArguments};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct LinearFunction<M: ManagedTypeApi> {
    pub initial_price: BigUint<M>,
    pub linear_coefficient: BigUint<M>,
}

impl<M: ManagedTypeApi> CurveFunction<M> for LinearFunction<M> {
    fn calculate_price(
        &self,
        token_start: &BigUint<M>,
        amount: &BigUint<M>,
        _arguments: &CurveArguments<M>,
    ) -> BigUint<M> {
        &self.linear_coefficient * &sum_interval(amount, token_start) + &self.initial_price * amount
    }
}

fn sum_interval<M: ManagedTypeApi>(n: &BigUint<M>, x: &BigUint<M>) -> BigUint<M> {
    x * n + &(n - 1u32) * n / 2u32
}
