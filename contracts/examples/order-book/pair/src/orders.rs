elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::common::{FEE_PENALTY_INCREASE_EPOCHS, FEE_PENALTY_INCREASE_PERCENT};

use super::{common, events, validation};

use super::common::{
    Order, OrderInputParams, OrderType, Payment, Transfer, FREE_ORDER_FROM_STORAGE_MIN_PENALTIES,
    PERCENT_BASE_POINTS,
};
use core::iter::FromIterator;

#[elrond_wasm::module]
pub trait OrdersModule:
    events::EventsModule + common::CommonModule + validation::ValidationModule
{
    fn create_order(
        &self,
        payment: Payment<Self::Api>,
        params: OrderInputParams<Self::Api>,
        order_type: OrderType,
    ) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();

        let mut address_order_ids = self.get_address_order_ids(caller).into_vec();
        self.require_not_max_size(&address_order_ids)?;

        let new_order_id = self.get_and_increase_order_id_counter();
        let order = self.new_order(new_order_id, payment, params, order_type);
        self.orders(order.id).set(&order);

        address_order_ids.push(order.id);
        self.address_order_ids(caller).set(&address_order_ids);

        self.emit_order_event(order);
        Ok(())
    }

    fn match_orders(&self, order_ids: Vec<u64>) -> SCResult<()> {
        let orders = self.load_orders(&order_ids);
        require!(
            orders.len() == order_ids.len(),
            "Order vectors len mismatch"
        );
        self.require_match_provider_empty_or_caller(&orders)?;

        let transfers = self.create_transfers(&orders)?;
        self.clear_orders(&order_ids);
        self.execute_transfers(transfers);

        self.emit_match_order_events(orders);
        Ok(())
    }

    fn cancel_all_orders(&self) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();
        let address_order_ids = self.get_address_order_ids(caller).into_vec();

        let order_ids_not_empty = address_order_ids
            .iter()
            .filter(|&&x| !self.orders(x).is_empty())
            .copied()
            .collect::<Vec<u64>>();

        self.cancel_orders(order_ids_not_empty)
    }

    fn cancel_orders(&self, order_ids: Vec<u64>) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();
        let mut address_order_ids = self.get_address_order_ids(caller).into_vec();
        self.require_contains_all(&address_order_ids, &order_ids)?;

        let first_token_id = &self.first_token_id().get();
        let second_token_id = &self.second_token_id().get();
        let epoch = self.blockchain().get_block_epoch();

        let order_ids_not_empty = order_ids
            .iter()
            .filter(|&&x| !self.orders(x).is_empty())
            .copied()
            .collect::<Vec<u64>>();

        let mut orders = Vec::new();
        for &order_id in order_ids_not_empty.iter() {
            let order =
                self.cancel_order(order_id, caller, first_token_id, second_token_id, epoch)?;
            address_order_ids.remove(order_id as usize);
            orders.push(order);
        }
        self.address_order_ids(caller).set(&address_order_ids);

        self.emit_cancel_order_events(orders);
        Ok(())
    }

    fn free_orders(&self, order_ids: Vec<u64>) -> SCResult<()> {
        let caller = &self.blockchain().get_caller();
        let address_order_ids = self.get_address_order_ids(caller).into_vec();
        self.require_contains_none(&address_order_ids, &order_ids)?;

        let first_token_id = &self.first_token_id().get();
        let second_token_id = &self.second_token_id().get();
        let epoch = self.blockchain().get_block_epoch();

        let order_ids_not_empty = order_ids
            .iter()
            .filter(|&&x| !self.orders(x).is_empty())
            .copied()
            .collect::<Vec<u64>>();

        let mut orders = Vec::new();
        for &order_id in order_ids_not_empty.iter() {
            let order =
                self.free_order(order_id, caller, first_token_id, second_token_id, epoch)?;
            orders.push(order);
        }

        self.emit_free_order_events(orders);
        Ok(())
    }

    fn free_order(
        &self,
        order_id: u64,
        caller: &ManagedAddress,
        first_token_id: &TokenIdentifier,
        second_token_id: &TokenIdentifier,
        epoch: u64,
    ) -> SCResult<Order<Self::Api>> {
        let order = self.orders(order_id).get();

        let token_id = match &order.order_type {
            OrderType::Buy => second_token_id.clone(),
            OrderType::Sell => first_token_id.clone(),
        };

        let penalty_count = (epoch - order.create_epoch) / FEE_PENALTY_INCREASE_EPOCHS;
        require!(
            penalty_count >= FREE_ORDER_FROM_STORAGE_MIN_PENALTIES,
            "Too early to free order"
        );

        let penalty_percent = penalty_count * FEE_PENALTY_INCREASE_PERCENT;
        let penalty_amount = self.rule_of_three(
            &BigUint::from(penalty_percent),
            &BigUint::from(PERCENT_BASE_POINTS),
            &order.input_amount,
        );
        let amount = &order.input_amount - &penalty_amount;

        let creator_transfer = Transfer {
            to: order.creator.clone(),
            payment: Payment {
                token_id: token_id.clone(),
                amount,
            },
        };
        let caller_transfer = Transfer {
            to: caller.clone(),
            payment: Payment {
                token_id,
                amount: penalty_amount,
            },
        };

        self.orders(order_id).clear();
        self.execute_transfers([creator_transfer, caller_transfer].to_vec());

        Ok(order)
    }

    fn cancel_order(
        &self,
        order_id: u64,
        caller: &ManagedAddress,
        first_token_id: &TokenIdentifier,
        second_token_id: &TokenIdentifier,
        epoch: u64,
    ) -> SCResult<Order<Self::Api>> {
        let order = self.orders(order_id).get();

        let token_id = match &order.order_type {
            OrderType::Buy => second_token_id.clone(),
            OrderType::Sell => first_token_id.clone(),
        };

        let penalty_count = (epoch - order.create_epoch) / FEE_PENALTY_INCREASE_EPOCHS;
        let penalty_percent = penalty_count * FEE_PENALTY_INCREASE_PERCENT;
        let penalty_amount = self.rule_of_three(
            &BigUint::from(penalty_percent),
            &BigUint::from(PERCENT_BASE_POINTS),
            &order.input_amount,
        );
        let amount = &order.input_amount - &penalty_amount;

        let transfer = Transfer {
            to: caller.clone(),
            payment: Payment { token_id, amount },
        };

        self.orders(order_id).clear();
        self.execute_transfers([transfer].to_vec());

        Ok(order)
    }

    fn load_orders(&self, order_ids: &[u64]) -> Vec<Order<Self::Api>> {
        order_ids
            .iter()
            .filter(|&x| !self.orders(*x).is_empty())
            .map(|&x| self.orders(x).get())
            .collect()
    }

    fn create_transfers(&self, orders: &[Order<Self::Api>]) -> SCResult<Vec<Transfer<Self::Api>>> {
        let mut transfers = Vec::new();
        let first_token_id = self.first_token_id().get();
        let second_token_id = self.second_token_id().get();

        let buy_orders = self.get_orders_with_type(orders, OrderType::Buy);
        let sell_orders = self.get_orders_with_type(orders, OrderType::Sell);

        let (second_token_paid, first_token_requested) = self.get_orders_sum_up(&buy_orders);
        let (first_token_paid, second_token_requested) = self.get_orders_sum_up(&sell_orders);
        require!(
            first_token_paid >= first_token_requested,
            "Orders mismatch: Not enough first Token"
        );
        require!(
            second_token_paid >= second_token_requested,
            "Orders mismatch: Not enough second Token"
        );

        let first_token_leftover = &first_token_paid - &first_token_requested;
        let second_token_leftover = &second_token_paid - &second_token_requested;

        let mut buyers_transfers = self.calculate_transfers(
            buy_orders,
            second_token_paid,
            first_token_id,
            first_token_leftover,
        );
        transfers.append(&mut buyers_transfers);

        let mut sellers_transfers = self.calculate_transfers(
            sell_orders,
            first_token_paid,
            second_token_id,
            second_token_leftover,
        );
        transfers.append(&mut sellers_transfers);

        Ok(transfers)
    }

    fn get_orders_with_type(
        &self,
        orders: &[Order<Self::Api>],
        order_type: OrderType,
    ) -> Vec<Order<Self::Api>> {
        orders
            .iter()
            .filter(|&x| x.order_type == order_type)
            .cloned()
            .collect()
    }

    fn get_orders_sum_up(&self, orders: &[Order<Self::Api>]) -> (BigUint, BigUint) {
        let mut amount_paid = BigUint::zero();
        let mut amount_requested = BigUint::zero();

        orders.iter().for_each(|x| {
            amount_paid += &x.input_amount;
            amount_requested += &x.output_amount;
        });

        (amount_paid, amount_requested)
    }

    fn calculate_transfers(
        &self,
        orders: Vec<Order<Self::Api>>,
        total_paid: BigUint,
        token_requested: TokenIdentifier,
        leftover: BigUint,
    ) -> Vec<Transfer<Self::Api>> {
        let mut transfers = Vec::new();

        let mut match_provider_transfer = Transfer {
            to: self.blockchain().get_caller(),
            payment: Payment {
                token_id: token_requested.clone(),
                amount: BigUint::zero(),
            },
        };

        for order in orders.iter() {
            let match_provider_amount =
                self.calculate_fee_amount(&order.output_amount, &order.fee_config);
            let creator_amount = &order.output_amount - &match_provider_amount;

            let order_deal = self.rule_of_three(&order.input_amount, &total_paid, &leftover);
            let match_provider_deal_amount = self.rule_of_three(
                &order.deal_config.match_provider_percent.into(),
                &PERCENT_BASE_POINTS.into(),
                &order_deal,
            );
            let creator_deal_amount = &order_deal - &match_provider_deal_amount;

            transfers.push(Transfer {
                to: order.creator.clone(),
                payment: Payment {
                    token_id: token_requested.clone(),
                    amount: creator_amount + creator_deal_amount,
                },
            });

            match_provider_transfer.payment.amount +=
                match_provider_amount + match_provider_deal_amount;
        }
        transfers.push(match_provider_transfer);

        transfers
    }

    fn execute_transfers(&self, transfers: Vec<Transfer<Self::Api>>) {
        for transfer in transfers {
            if transfer.payment.amount > 0 {
                self.send().direct(
                    &transfer.to,
                    &transfer.payment.token_id,
                    0,
                    &transfer.payment.amount,
                    &[],
                )
            }
        }
    }

    fn clear_orders(&self, order_ids: &[u64]) {
        order_ids.iter().for_each(|&x| self.orders(x).clear())
    }

    fn get_and_increase_order_id_counter(&self) -> u64 {
        let id = self.order_id_counter().get();
        self.order_id_counter().set(&(id + 1));
        id
    }

    #[view(getAddressOrderIds)]
    fn get_address_order_ids(&self, address: &ManagedAddress) -> MultiResultVec<u64> {
        MultiResultVec::from_iter(
            self.address_order_ids(address)
                .get()
                .iter()
                .filter(|&&x| !self.orders(x).is_empty())
                .copied()
                .collect::<Vec<u64>>(),
        )
    }

    #[view(getOrderIdCounter)]
    #[storage_mapper("order_id_counter")]
    fn order_id_counter(&self) -> SingleValueMapper<u64>;

    #[view(getOrderById)]
    #[storage_mapper("orders")]
    fn orders(&self, id: u64) -> SingleValueMapper<Order<Self::Api>>;

    #[storage_mapper("address_order_ids")]
    fn address_order_ids(&self, address: &ManagedAddress) -> SingleValueMapper<Vec<u64>>;
}
