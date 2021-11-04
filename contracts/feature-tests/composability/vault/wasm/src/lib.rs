////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    vault::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    vault::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds() {
    vault::endpoints::accept_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds_echo_payment() {
    vault::endpoints::accept_funds_echo_payment(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_funds_multi_transfer() {
    vault::endpoints::accept_funds_multi_transfer(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn accept_multi_funds_echo() {
    vault::endpoints::accept_multi_funds_echo(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn burn_and_create_retrive_async() {
    vault::endpoints::burn_and_create_retrive_async(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_counts() {
    vault::endpoints::call_counts(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_arguments() {
    vault::endpoints::echo_arguments(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_caller() {
    vault::endpoints::echo_caller(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn get_owner_address() {
    vault::endpoints::get_owner_address(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn just_accept_funds() {
    vault::endpoints::just_accept_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn reject_funds() {
    vault::endpoints::reject_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn retrieve_funds() {
    vault::endpoints::retrieve_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn retrieve_multi_funds_async() {
    vault::endpoints::retrieve_multi_funds_async(elrond_wasm_node::vm_api());
}
