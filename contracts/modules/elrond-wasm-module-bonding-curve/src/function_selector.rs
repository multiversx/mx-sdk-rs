elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    curves::{curve_function::CurveFunction, linear_function::LinearFunction},
    utils::structs::CurveArguments,
};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Clone)]
pub enum FunctionSelector<M: ManagedTypeApi> {
    Linear(LinearFunction<M>),
    CustomExample(BigUint<M>),
    None,
}

impl<M: ManagedTypeApi> FunctionSelector<M> {
    pub fn is_none(&self) -> bool {
        matches!(self, FunctionSelector::None)
    }
}

impl<M: ManagedTypeApi> CurveFunction<M> for FunctionSelector<M> {
    fn calculate_price(
        &self,
        token_start: &BigUint<M>,
        amount: &BigUint<M>,
        arguments: &CurveArguments<M>,
    ) -> SCResult<BigUint<M>> {
        match &self {
            FunctionSelector::Linear(linear_function) => {
                CurveFunction::calculate_price(linear_function, token_start, amount, arguments)
            },

            FunctionSelector::CustomExample(initial_cost) => {
                let sum = token_start + amount;
                let price = &(&sum * &sum * sum / 3u32) + &arguments.balance + initial_cost.clone();
                Ok(price)
            },
            FunctionSelector::None => Err("Bonding Curve function is not assiged".into()),
        }
    }
}
