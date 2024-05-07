use crate::{
    api::{ErrorApiImpl, ManagedTypeApi},
    types::BigUint,
};

pub trait FixedSupplyToken<M: ManagedTypeApi> {
    fn get_total_supply(&self) -> BigUint<M>;

    fn into_part(self, payment_amount: &BigUint<M>) -> Self;

    /// full_value * current_supply / total_supply
    fn rule_of_three(&self, current_supply: &BigUint<M>, full_value: &BigUint<M>) -> BigUint<M> {
        let total_supply = self.get_total_supply();
        if current_supply == &total_supply {
            return full_value.clone();
        }

        (full_value * current_supply) / total_supply
    }

    /// full_value * current_supply / total_supply
    fn rule_of_three_non_zero_result(
        &self,
        current_supply: &BigUint<M>,
        full_value: &BigUint<M>,
    ) -> BigUint<M> {
        let result = self.rule_of_three(current_supply, full_value);
        if result == 0 {
            M::error_api_impl().signal_error(b"Zero amount");
        }

        result
    }
}
