////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    proxy_test_first::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    proxy_test_first::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn deploySecondContract() {
    proxy_test_first::endpoints::deploySecondContract(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forwardToOtherContract() {
    proxy_test_first::endpoints::forwardToOtherContract(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn forwardToOtherContractWithCallback() {
    proxy_test_first::endpoints::forwardToOtherContractWithCallback(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn messageOtherContract() {
    proxy_test_first::endpoints::messageOtherContract(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn messageOtherContractWithCallback() {
    proxy_test_first::endpoints::messageOtherContractWithCallback(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn upgradeSecondContract() {
    proxy_test_first::endpoints::upgradeSecondContract(elrond_wasm_node::vm_api());
}
