////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    local_esdt_and_nft::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    local_esdt_and_nft::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getCurrentNftNonce() {
    local_esdt_and_nft::endpoints::getCurrentNftNonce(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getFungibleEsdtBalance() {
    local_esdt_and_nft::endpoints::getFungibleEsdtBalance(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getNftBalance() {
    local_esdt_and_nft::endpoints::getNftBalance(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn issueFungibleToken() {
    local_esdt_and_nft::endpoints::issueFungibleToken(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastErrorMessage() {
    local_esdt_and_nft::endpoints::lastErrorMessage(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn lastIssuedToken() {
    local_esdt_and_nft::endpoints::lastIssuedToken(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn localBurn() {
    local_esdt_and_nft::endpoints::localBurn(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn localMint() {
    local_esdt_and_nft::endpoints::localMint(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftAddQuantity() {
    local_esdt_and_nft::endpoints::nftAddQuantity(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftBurn() {
    local_esdt_and_nft::endpoints::nftBurn(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftCreate() {
    local_esdt_and_nft::endpoints::nftCreate(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn nftIssue() {
    local_esdt_and_nft::endpoints::nftIssue(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    local_esdt_and_nft::endpoints::setLocalRoles(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn sftIssue() {
    local_esdt_and_nft::endpoints::sftIssue(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transferNftViaAsyncCall() {
    local_esdt_and_nft::endpoints::transferNftViaAsyncCall(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_nft_and_execute() {
    local_esdt_and_nft::endpoints::transfer_nft_and_execute(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    local_esdt_and_nft::endpoints::unsetLocalRoles(elrond_wasm_node::vm_api());
}
