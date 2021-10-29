////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    parent::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deposit() {
    parent::endpoints::deposit(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deployChildContract() {
    parent::endpoints::deployChildContract(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn executeOnDestIssueToken() {
    parent::endpoints::executeOnDestIssueToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getChildContractAddress() {
    parent::endpoints::getChildContractAddress(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    parent::endpoints::callBack(elrond_wasm_node::arwen_api());
}
