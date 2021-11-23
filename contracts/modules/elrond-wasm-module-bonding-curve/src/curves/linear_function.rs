elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::curves::curve_function::CurveFunction;

use crate::utils::structs::CurveArguments;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
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
    ) -> SCResult<BigUint<M>> {
        Ok(
            &self.linear_coefficient * &sum_interval(amount, token_start)
                + &self.initial_price * amount,
        )
    }
}
fn sum_interval<M: ManagedTypeApi>(n: &BigUint<M>, x: &BigUint<M>) -> BigUint<M> {
    x * n + &(n - 1u32) * n / 2u32
}
