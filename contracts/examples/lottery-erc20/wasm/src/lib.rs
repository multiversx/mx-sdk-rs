////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    lottery_erc20::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn start() {
    lottery_erc20::endpoints::start(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createLotteryPool() {
    lottery_erc20::endpoints::createLotteryPool(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn buy_ticket() {
    lottery_erc20::endpoints::buy_ticket(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn determine_winner() {
    lottery_erc20::endpoints::determine_winner(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn status() {
    lottery_erc20::endpoints::status(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn lotteryInfo() {
    lottery_erc20::endpoints::lotteryInfo(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn erc20ContractManagedAddress() {
    lottery_erc20::endpoints::erc20ContractManagedAddress(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    lottery_erc20::endpoints::callBack(elrond_wasm_node::arwen_api());
}
