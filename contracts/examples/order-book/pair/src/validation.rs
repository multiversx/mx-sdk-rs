multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::common::{FeeConfig, FeeConfigEnum};

use super::{
    common,
    common::{
        Order, OrderInputParams, Payment, FEE_PENALTY_INCREASE_PERCENT, MAX_ORDERS_PER_USER,
        PERCENT_BASE_POINTS,
    },
};

#[multiversx_sc::module]
pub trait ValidationModule: common::CommonModule {
    fn require_valid_order_input_amount(&self, params: &OrderInputParams<Self::Api>) {
        require!(params.amount != BigUint::zero(), "Amout cannot be zero");
        require!(
            self.calculate_fee_amount(
                &params.amount,
                &FeeConfig {
                    fee_type: FeeConfigEnum::Percent,
                    fixed_fee: BigUint::zero(),
                    percent_fee: FEE_PENALTY_INCREASE_PERCENT,
                }
            ) != BigUint::zero(),
            "Penalty increase amount cannot be zero"
        );
    }

    fn require_valid_order_input_match_provider(&self, params: &OrderInputParams<Self::Api>) {
        require!(
            !params.match_provider.clone().is_zero(),
            "Match address cannot be zero"
        );
    }

    fn require_valid_order_input_fee_config(&self, params: &OrderInputParams<Self::Api>) {
        match params.fee_config.fee_type.clone() {
            FeeConfigEnum::Fixed => {
                require!(
                    params.fee_config.fixed_fee < params.amount,
                    "Invalid fee config fixed amount"
                );
            },
            FeeConfigEnum::Percent => {
                require!(
                    params.fee_config.percent_fee < PERCENT_BASE_POINTS,
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
        let (token_id, amount) = self.call_value().single_fungible_esdt();
        let second_token_id = self.second_token_id().get();
        require!(
            token_id == second_token_id,
            "Token in and second token id should be the same"
        );

        Payment { token_id, amount }
    }

    fn require_valid_sell_payment(&self) -> Payment<Self::Api> {
        let (token_id, amount) = self.call_value().single_fungible_esdt();
        let first_token_id = self.first_token_id().get();
        require!(
            token_id == first_token_id,
            "Token in and first token id should be the same"
        );

        Payment { token_id, amount }
    }

    fn require_valid_match_input_order_ids(&self, order_ids: &ManagedVec<u64>) {
        require!(order_ids.len() >= 2, "Should be at least two order ids");
    }

    fn require_not_max_size(&self, address_order_ids: &MultiValueManagedVec<u64>) {
        require!(
            address_order_ids.len() < MAX_ORDERS_PER_USER,
            "Cannot place more orders"
        );
    }

    fn require_order_ids_not_empty(&self, order_ids: &MultiValueManagedVec<u64>) {
        require!(!order_ids.is_empty(), "Order ids vec is empty");
    }

    fn require_match_provider_empty_or_caller(
        &self,
        orders: &MultiValueManagedVec<Order<Self::Api>>,
    ) {
        let caller = &self.blockchain().get_caller();

        for order in orders.iter() {
            if order.match_provider != ManagedAddress::zero() {
                require!(
                    &order.match_provider == caller,
                    "Caller is not matched order id"
                );
            } else {
                {}
            }
        }
    }

    fn require_contains_all(
        &self,
        vec_base: &MultiValueManagedVec<u64>,
        items: &MultiValueManagedVec<u64>,
    ) {
        for item in items.iter() {
            let mut check_item = false;
            for base in vec_base.iter() {
                if item == base {
                    check_item = true;
                    break;
                }
            }
            require!(check_item, "Base vec does not contain item");
        }
    }

    fn require_contains_none(
        &self,
        vec_base: &MultiValueManagedVec<u64>,
        items: &MultiValueManagedVec<u64>,
    ) {
        for item in items.iter() {
            let mut check_item = false;
            for base in vec_base.iter() {
                if item == base {
                    check_item = true;
                    break;
                }
            }
            require!(!check_item, "Base vec contains item");
        }
    }
}
