////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    abi_tester::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    abi_tester::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn address_vs_h256() {
    abi_tester::endpoints::address_vs_h256(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_abi_test_type() {
    abi_tester::endpoints::echo_abi_test_type(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn echo_enum() {
    abi_tester::endpoints::echo_enum(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn esdt_local_role() {
    abi_tester::endpoints::esdt_local_role(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn esdt_token_data() {
    abi_tester::endpoints::esdt_token_data(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn esdt_token_payment() {
    abi_tester::endpoints::esdt_token_payment(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn managed_address_vs_byte_array() {
    abi_tester::endpoints::managed_address_vs_byte_array(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_3() {
    abi_tester::endpoints::multi_result_3(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_4() {
    abi_tester::endpoints::multi_result_4(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn multi_result_vec() {
    abi_tester::endpoints::multi_result_vec(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn optional_arg() {
    abi_tester::endpoints::optional_arg(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn optional_result() {
    abi_tester::endpoints::optional_result(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_any_token() {
    abi_tester::endpoints::payable_any_token(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_egld() {
    abi_tester::endpoints::payable_egld(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn payable_some_token() {
    abi_tester::endpoints::payable_some_token(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn sample_storage_mapper() {
    abi_tester::endpoints::sample_storage_mapper(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn var_args() {
    abi_tester::endpoints::var_args(elrond_wasm_node::vm_api());
}
