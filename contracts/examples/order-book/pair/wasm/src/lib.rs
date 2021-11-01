////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    order_book_pair::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    order_book_pair::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn cancelAllOrders() {
    order_book_pair::endpoints::cancelAllOrders(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn cancelOrders() {
    order_book_pair::endpoints::cancelOrders(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createBuyOrder() {
    order_book_pair::endpoints::createBuyOrder(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createSellOrder() {
    order_book_pair::endpoints::createSellOrder(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn freeOrders() {
    order_book_pair::endpoints::freeOrders(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getAddressOrderIds() {
    order_book_pair::endpoints::getAddressOrderIds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getFirstTokenId() {
    order_book_pair::endpoints::getFirstTokenId(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getOrderById() {
    order_book_pair::endpoints::getOrderById(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getOrderIdCounter() {
    order_book_pair::endpoints::getOrderIdCounter(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getSecondTokenId() {
    order_book_pair::endpoints::getSecondTokenId(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn matchOrders() {
    order_book_pair::endpoints::matchOrders(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn startGlobalOperation() {
    order_book_pair::endpoints::startGlobalOperation(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn stopGlobalOperation() {
    order_book_pair::endpoints::stopGlobalOperation(elrond_wasm_node::arwen_api());
}
