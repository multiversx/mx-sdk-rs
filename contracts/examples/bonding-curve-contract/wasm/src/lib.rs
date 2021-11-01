////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    bonding_curve_contract::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    bonding_curve_contract::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn buyToken() {
    bonding_curve_contract::endpoints::buyToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn claim() {
    bonding_curve_contract::endpoints::claim(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deposit() {
    bonding_curve_contract::endpoints::deposit(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTokenAvailability() {
    bonding_curve_contract::endpoints::getTokenAvailability(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_buy_price() {
    bonding_curve_contract::endpoints::get_buy_price(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_sell_price() {
    bonding_curve_contract::endpoints::get_sell_price(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn sellToken() {
    bonding_curve_contract::endpoints::sellToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setBondingCurve() {
    bonding_curve_contract::endpoints::setBondingCurve(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    bonding_curve_contract::endpoints::setLocalRoles(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    bonding_curve_contract::endpoints::unsetLocalRoles(elrond_wasm_node::arwen_api());
}
