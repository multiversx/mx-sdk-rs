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
    fn require_valid_order_input_amount(&self, params: &OrderInputParams<Self::Api>) {
        require!(params.amount != 0, "Amout cannot be zero");
        require!(
            self.calculate_fee_amount(
                &params.amount,
                &FeeConfig::Percent(FEE_PENALTY_INCREASE_PERCENT)
            ) != 0,
            "Penalty increase amount cannot be zero"
        );
    }

    fn require_valid_order_input_match_provider(&self, params: &OrderInputParams<Self::Api>) {
        require!(
            params.match_provider.is_none() || !params.match_provider.clone().unwrap().is_zero(),
            "Match address cannot be zero"
        );
    }

    fn require_valid_order_input_fee_config(&self, params: &OrderInputParams<Self::Api>) {
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
    }

    fn require_valid_order_input_deal_config(&self, params: &OrderInputParams<Self::Api>) {
        require!(
            params.deal_config.match_provider_percent < PERCENT_BASE_POINTS,
            "Bad deal config"
        );
    }

    fn require_valid_order_input_params(&self, params: &OrderInputParams<Self::Api>) {
        self.require_valid_order_input_amount(params);
        self.require_valid_order_input_match_provider(params);
        self.require_valid_order_input_fee_config(params);
        self.require_valid_order_input_deal_config(params);
    }

    fn require_valid_buy_payment(&self) -> Payment<Self::Api> {
        self.require_fungible_input();
        let second_token_id = self.second_token_id().get();
        let (amount, token_id) = self.call_value().payment_token_pair();
        require!(
            token_id == second_token_id,
            "Token in and second token id should be the same"
        );
        require!(amount != 0, "Input amount should not be zero");

        Payment { token_id, amount }
    }

    fn require_valid_sell_payment(&self) -> Payment<Self::Api> {
        self.require_fungible_input();
        let first_token_id = self.first_token_id().get();
        let (amount, token_id) = self.call_value().payment_token_pair();
        require!(
            token_id == first_token_id,
            "Token in and first token id should be the same"
        );
        require!(amount != 0, "Input amount should not be zero");

        Payment { token_id, amount }
    }

    fn require_valid_match_input_order_ids(&self, order_ids: &[u64]) {
        require!(order_ids.len() >= 2, "Should be at least two order ids");
    }

    fn require_fungible_input(&self) {
        require!(
            self.call_value().esdt_token_nonce() == 0,
            "Nonce is not zero"
        );
    }

    fn require_not_max_size(&self, address_order_ids: &[u64]) {
        require!(
            address_order_ids.len() < MAX_ORDERS_PER_USER,
            "Cannot place more orders"
        );
    }

    fn require_order_ids_not_empty(&self, order_ids: &[u64]) {
        require!(!order_ids.is_empty(), "Order ids vec is empty");
    }

    fn require_match_provider_empty_or_caller(&self, orders: &[Order<Self::Api>]) {
        let caller = &self.blockchain().get_caller();

        for order in orders.iter() {
            match &order.match_provider {
                Some(address) => {
                    require!(address == caller, "Caller is not matched order id");
                },
                None => {},
            }
        }
    }

    fn require_contains_all(&self, vec_base: &[u64], items: &[u64]) {
        for item in items.iter() {
            require!(vec_base.contains(item), "Base vec do not contain item");
        }
    }

    fn require_contains_none(&self, vec_base: &[u64], items: &[u64]) {
        for item in items.iter() {
            require!(!vec_base.contains(item), "Base vec contains item");
        }
    }
}
