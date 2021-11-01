////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    crypto_bubbles::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    crypto_bubbles::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    crypto_bubbles::endpoints::balanceOf(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn joinGame() {
    crypto_bubbles::endpoints::joinGame(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn rewardAndSendToWallet() {
    crypto_bubbles::endpoints::rewardAndSendToWallet(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn rewardWinner() {
    crypto_bubbles::endpoints::rewardWinner(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn topUp() {
    crypto_bubbles::endpoints::topUp(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn withdraw() {
    crypto_bubbles::endpoints::withdraw(elrond_wasm_node::vm_api());
}
