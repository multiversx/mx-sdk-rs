#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod common;
mod events;
mod global;
mod orders;
mod validation;

use common::OrderInputParams;

#[multiversx_sc::contract]
pub trait Pair:
    global::GlobalOperationModule
    + orders::OrdersModule
    + events::EventsModule
    + common::CommonModule
    + validation::ValidationModule
{
    #[init]
    fn init(&self, first_token_id: TokenIdentifier, second_token_id: TokenIdentifier) {
        self.first_token_id().set_if_empty(&first_token_id);
        self.second_token_id().set_if_empty(&second_token_id);
    }

    #[payable("*")]
    #[endpoint(createBuyOrder)]
    fn create_buy_order_endpoint(&self, params: OrderInputParams<Self::Api>) {
        self.require_global_op_not_ongoing();
        self.require_valid_order_input_params(&params);
        let payment = self.require_valid_buy_payment();

        self.create_order(payment, params, common::OrderType::Buy);
    }

    #[payable("*")]
    #[endpoint(createSellOrder)]
    fn create_sell_order_endpoint(&self, params: OrderInputParams<Self::Api>) {
        self.require_global_op_not_ongoing();
        self.require_valid_order_input_params(&params);
        let payment = self.require_valid_sell_payment();

        self.create_order(payment, params, common::OrderType::Sell);
    }

    #[endpoint(matchOrders)]
    fn match_orders_endpoint(&self, order_ids: ManagedVec<u64>) {
        self.require_global_op_not_ongoing();
        self.require_valid_match_input_order_ids(&order_ids);

        self.match_orders(order_ids);
    }

    #[endpoint(cancelOrders)]
    fn cancel_orders_endpoint(&self, order_ids: MultiValueManagedVec<u64>) {
        self.require_global_op_not_ongoing();
        self.require_order_ids_not_empty(&order_ids);

        self.cancel_orders(order_ids);
    }

    #[endpoint(cancelAllOrders)]
    fn cancel_all_orders_endpoint(&self) {
        self.require_global_op_not_ongoing();
        self.cancel_all_orders();
    }

    #[endpoint(freeOrders)]
    fn free_orders_endpoint(&self, order_ids: MultiValueManagedVec<u64>) {
        self.require_global_op_not_ongoing();
        self.require_order_ids_not_empty(&order_ids);

        self.free_orders(order_ids);
    }
}
