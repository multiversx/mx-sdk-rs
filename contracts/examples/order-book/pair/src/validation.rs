elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::common::FeeConfig;

use super::{
    common,
    common::{
        Order, OrderInputParams, Payment, FEE_PENALTY_INCREASE_PERCENT, MAX_ORDERS_PER_USER,
        PERCENT_BASE_POINTS,
    },
};

#[elrond_wasm::module]
pub trait ValidationModule: common::CommonModule {
    fn require_valid_order_input_amount(
        &self,
        params: &OrderInputParams<Self::Api>,
    ) -> SCResult<()> {
        require!(params.amount != 0, "Amout cannot be zero");
        require!(
            self.calculate_fee_amount(
                &params.amount,
                &FeeConfig::Percent(FEE_PENALTY_INCREASE_PERCENT)
            ) != 0,
            "Penalty increase amount cannot be zero"
        );
        Ok(())
    }

    fn require_valid_order_input_match_provider(
        &self,
        params: &OrderInputParams<Self::Api>,
    ) -> SCResult<()> {
        require!(
            params.match_provider.is_none() || !params.match_provider.clone().unwrap().is_zero(),
            "Match address cannot be zero"
        );
        Ok(())
    }

    fn require_valid_order_input_fee_config(
        &self,
        params: &OrderInputParams<Self::Api>,
    ) -> SCResult<()> {
        match params.fee_config.clone() {
            FeeConfig::Fixed(amount) => {
                require!(amount < params.amount, "Invalid fee config fixed amount");
            },
            FeeConfig::Percent(percent) => {
                require!(
                    percent < PERCENT_BASE_POINTS,
                    "Percent value above maximum value"
                );
            },
        }

        let amount_after_fee = self.calculate_amount_after_fee(&params.amount, &params.fee_config);
        require!(amount_after_fee != 0, "Amount after fee cannot be zero");
        Ok(())
    }

    fn require_valid_order_input_deal_config(
        &self,
        params: &OrderInputParams<Self::Api>,
    ) -> SCResult<()> {
        require!(
            params.deal_config.match_provider_percent < PERCENT_BASE_POINTS,
            "Bad deal config"
        );
        Ok(())
    }

    fn require_valid_order_input_params(
        &self,
        params: &OrderInputParams<Self::Api>,
    ) -> SCResult<()> {
        self.require_valid_order_input_amount(params)?;
        self.require_valid_order_input_match_provider(params)?;
        self.require_valid_order_input_fee_config(params)?;
        self.require_valid_order_input_deal_config(params)?;
        Ok(())
    }

    fn require_valid_buy_payment(&self) -> SCResult<Payment<Self::Api>> {
        self.require_fungible_input()?;
        let second_token_id = self.second_token_id().get();
        let (amount, token_id) = self.call_value().payment_token_pair();
        require!(
            token_id == second_token_id,
            "Token in and second token id should be the same"
        );
        require!(amount != 0, "Input amount should not be zero");
        Ok(Payment { token_id, amount })
    }

    fn require_valid_sell_payment(&self) -> SCResult<Payment<Self::Api>> {
        self.require_fungible_input()?;
        let first_token_id = self.first_token_id().get();
        let (amount, token_id) = self.call_value().payment_token_pair();
        require!(
            token_id == first_token_id,
            "Token in and first token id should be the same"
        );
        require!(amount != 0, "Input amount should not be zero");
        Ok(Payment { token_id, amount })
    }

    fn require_valid_match_input_order_ids(&self, order_ids: &[u64]) -> SCResult<()> {
        require!(order_ids.len() >= 2, "Should be at least two order ids");
        Ok(())
    }

    fn require_fungible_input(&self) -> SCResult<()> {
        require!(
            self.call_value().esdt_token_nonce() == 0,
            "Nonce is not zero"
        );
        Ok(())
    }

    fn require_not_max_size(&self, address_order_ids: &[u64]) -> SCResult<()> {
        require!(
            address_order_ids.len() < MAX_ORDERS_PER_USER,
            "Cannot place more orders"
        );
        Ok(())
    }

    fn require_order_ids_not_empty(&self, order_ids: &[u64]) -> SCResult<()> {
        require!(!order_ids.is_empty(), "Order ids vec is empty");
        Ok(())
    }

    fn require_match_provider_empty_or_caller(&self, orders: &[Order<Self::Api>]) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();

        for order in orders.iter() {
            match &order.match_provider {
                Some(address) => {
                    require!(address == caller, "Caller is not matched order id");
                },
                None => {},
            }
        }
        Ok(())
    }

    fn require_contains_all(&self, vec_base: &[u64], items: &[u64]) -> SCResult<()> {
        for item in items.iter() {
            require!(vec_base.contains(item), "Base vec do not contain item");
        }
        Ok(())
    }

    fn require_contains_none(&self, vec_base: &[u64], items: &[u64]) -> SCResult<()> {
        for item in items.iter() {
            require!(!vec_base.contains(item), "Base vec contains item");
        }
        Ok(())
    }
}
