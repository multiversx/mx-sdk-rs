////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    erc1155::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn safeTransferFrom() {
    erc1155::endpoints::safeTransferFrom(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn safeBatchTransferFrom() {
    erc1155::endpoints::safeBatchTransferFrom(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setApprovalForAll() {
    erc1155::endpoints::setApprovalForAll(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn createToken() {
    erc1155::endpoints::createToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn mint() {
    erc1155::endpoints::mint(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn burn() {
    erc1155::endpoints::burn(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn balanceOf() {
    erc1155::endpoints::balanceOf(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn balanceOfBatch() {
    erc1155::endpoints::balanceOfBatch(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTokenOwner() {
    erc1155::endpoints::getTokenOwner(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTokenTypeCreator() {
    erc1155::endpoints::getTokenTypeCreator(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getTokenTypeUri() {
    erc1155::endpoints::getTokenTypeUri(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn isFungible() {
    erc1155::endpoints::isFungible(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn isApprovedForAll() {
    erc1155::endpoints::isApprovedForAll(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    erc1155::endpoints::callBack(elrond_wasm_node::arwen_api());
}
