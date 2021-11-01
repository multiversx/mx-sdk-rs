////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    crowdfunding_esdt::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    crowdfunding_esdt::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    crowdfunding_esdt::endpoints::claim(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn fund() {
    crowdfunding_esdt::endpoints::fund(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCrowdfundingTokenIdentifier() {
    crowdfunding_esdt::endpoints::getCrowdfundingTokenIdentifier(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentFunds() {
    crowdfunding_esdt::endpoints::getCurrentFunds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDeadline() {
    crowdfunding_esdt::endpoints::getDeadline(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getDeposit() {
    crowdfunding_esdt::endpoints::getDeposit(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getTarget() {
    crowdfunding_esdt::endpoints::getTarget(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn status() {
    crowdfunding_esdt::endpoints::status(elrond_wasm_node::vm_api());
}
