////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    forwarder_raw::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    forwarder_raw::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context() {
    forwarder_raw::endpoints::call_execute_on_dest_context(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_by_caller() {
    forwarder_raw::endpoints::call_execute_on_dest_context_by_caller(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_readonly() {
    forwarder_raw::endpoints::call_execute_on_dest_context_readonly(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_dest_context_twice() {
    forwarder_raw::endpoints::call_execute_on_dest_context_twice(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn call_execute_on_same_context() {
    forwarder_raw::endpoints::call_execute_on_same_context(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callback_data() {
    forwarder_raw::endpoints::callback_data(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn clear_callback_info() {
    forwarder_raw::endpoints::clear_callback_info(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_contract() {
    forwarder_raw::endpoints::deploy_contract(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploy_from_source() {
    forwarder_raw::endpoints::deploy_from_source(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_call() {
    forwarder_raw::endpoints::forward_async_call(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_call_half_payment() {
    forwarder_raw::endpoints::forward_async_call_half_payment(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_async_retrieve_multi_transfer_funds() {
    forwarder_raw::endpoints::forward_async_retrieve_multi_transfer_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_direct_esdt_via_transf_exec() {
    forwarder_raw::endpoints::forward_direct_esdt_via_transf_exec(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_payment() {
    forwarder_raw::endpoints::forward_payment(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec() {
    forwarder_raw::endpoints::forward_transf_exec(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_egld() {
    forwarder_raw::endpoints::forward_transf_exec_egld(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forward_transf_exec_esdt() {
    forwarder_raw::endpoints::forward_transf_exec_esdt(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forwarder_async_send_and_retrieve_multi_transfer_funds() {
    forwarder_raw::endpoints::forwarder_async_send_and_retrieve_multi_transfer_funds(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgrade() {
    forwarder_raw::endpoints::upgrade(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgrade_from_source() {
    forwarder_raw::endpoints::upgrade_from_source(elrond_wasm_node::vm_api());
}
