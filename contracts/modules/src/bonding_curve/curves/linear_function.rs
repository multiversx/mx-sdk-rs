multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::{curves::curve_function::CurveFunction, utils::structs::CurveArguments};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone)]
pub struct LinearFunction<'a, M: ManagedTypeApi<'a>> {
    pub initial_price: BigUint<'a, M>,
    pub linear_coefficient: BigUint<'a, M>,
}

impl<'a, M: ManagedTypeApi<'a>> CurveFunction<'a, M> for LinearFunction<'a, M> {
    fn calculate_price(
        &self,
        token_start: &BigUint<'a, M>,
        amount: &BigUint<'a, M>,
        _arguments: &CurveArguments<'a, M>,
    ) -> BigUint<'a, M> {
        &self.linear_coefficient * &sum_interval(amount, token_start) + &self.initial_price * amount
    }
}

fn sum_interval<'a, M: ManagedTypeApi<'a>>(n: &BigUint<'a, M>, x: &BigUint<'a, M>) -> BigUint<'a, M> {
    x * n + &(n - 1u32) * n / 2u32
}
