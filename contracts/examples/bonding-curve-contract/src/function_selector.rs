multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::bonding_curve::{
    curves::{curve_function::CurveFunction, linear_function::LinearFunction},
    utils::structs::CurveArguments,
};

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq, Eq, Clone, Default,
)]
pub enum FunctionSelector<M: ManagedTypeApi> {
    Linear(LinearFunction<M>),
    CustomExample(BigUint<M>),
    #[default]
    None,
}

impl<M: ManagedTypeApi> CurveFunction<M> for FunctionSelector<M> {
    fn calculate_price(
        &self,
        token_start: &BigUint<M>,
        amount: &BigUint<M>,
        arguments: &CurveArguments<M>,
    ) -> BigUint<M> {
        match &self {
            FunctionSelector::Linear(linear_function) => {
                linear_function.calculate_price(token_start, amount, arguments)
            },

            FunctionSelector::CustomExample(initial_cost) => {
                let sum = token_start + amount;
                &(&sum * &sum * sum / 3u32) + &arguments.balance + initial_cost.clone()
            },
            FunctionSelector::None => {
                M::error_api_impl().signal_error(b"Bonding Curve function is not assiged")
            },
        }
    }
}
