////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155_user_mock::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155_user_mock::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn onERC1155BatchReceived() {
    erc1155_user_mock::endpoints::onERC1155BatchReceived(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn onERC1155Received() {
    erc1155_user_mock::endpoints::onERC1155Received(elrond_wasm_node::arwen_api());
}
