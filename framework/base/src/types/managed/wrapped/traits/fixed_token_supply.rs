use crate::imports::{BigUint, ErrorApiImpl, ManagedTypeApi};

pub trait FixedSupplyToken<'a, M: ManagedTypeApi<'a>> {
    fn get_total_supply(&self) -> BigUint<'a, M>;

    fn into_part(self, payment_amount: &BigUint<'a, M>) -> Self;

    /// full_value * current_supply / total_supply
    fn rule_of_three(&self, current_supply: &BigUint<'a, M>, full_value: &BigUint<'a, M>) -> BigUint<'a, M> {
        let total_supply = self.get_total_supply();
        if current_supply == &total_supply {
            return full_value.clone();
        }

        (full_value * current_supply) / total_supply
    }

    /// full_value * current_supply / total_supply
    fn rule_of_three_non_zero_result(
        &self,
        current_supply: &BigUint<'a, M>,
        full_value: &BigUint<'a, M>,
    ) -> BigUint<'a, M> {
        let result = self.rule_of_three(current_supply, full_value);
        if result == 0 {
            M::error_api_impl().signal_error(b"Zero amount");
        }

        result
    }
}
