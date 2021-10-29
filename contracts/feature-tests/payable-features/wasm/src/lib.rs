////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    payable_features::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn echo_call_value() {
    payable_features::endpoints::echo_call_value(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payment_multiple() {
    payable_features::endpoints::payment_multiple(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_any_1() {
    payable_features::endpoints::payable_any_1(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_any_2() {
    payable_features::endpoints::payable_any_2(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_any_3() {
    payable_features::endpoints::payable_any_3(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_any_4() {
    payable_features::endpoints::payable_any_4(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_egld_1() {
    payable_features::endpoints::payable_egld_1(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_egld_2() {
    payable_features::endpoints::payable_egld_2(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_egld_3() {
    payable_features::endpoints::payable_egld_3(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_egld_4() {
    payable_features::endpoints::payable_egld_4(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_token_1() {
    payable_features::endpoints::payable_token_1(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_token_2() {
    payable_features::endpoints::payable_token_2(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_token_3() {
    payable_features::endpoints::payable_token_3(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn payable_token_4() {
    payable_features::endpoints::payable_token_4(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    payable_features::endpoints::callBack(elrond_wasm_node::arwen_api());
}
