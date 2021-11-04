////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    first_contract::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    first_contract::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getSecondContractAddress() {
    first_contract::endpoints::getSecondContractAddress(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getesdtTokenName() {
    first_contract::endpoints::getesdtTokenName(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractFull() {
    first_contract::endpoints::transferToSecondContractFull(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractFullWithTransferAndExecute() {
    first_contract::endpoints::transferToSecondContractFullWithTransferAndExecute(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractHalf() {
    first_contract::endpoints::transferToSecondContractHalf(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractRejected() {
    first_contract::endpoints::transferToSecondContractRejected(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferToSecondContractRejectedWithTransferAndExecute() {
    first_contract::endpoints::transferToSecondContractRejectedWithTransferAndExecute(elrond_wasm_node::vm_api());
}
