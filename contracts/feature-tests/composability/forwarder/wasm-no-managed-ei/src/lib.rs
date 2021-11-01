////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    forwarder::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    forwarder::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn buy_nft() {
    forwarder::endpoints::buy_nft(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callback_data() {
    forwarder::endpoints::callback_data(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callback_data_at_index() {
    forwarder::endpoints::callback_data_at_index(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn changeOwnerAddress() {
    forwarder::endpoints::changeOwnerAddress(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn clear_callback_data() {
    forwarder::endpoints::clear_callback_data(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn create_and_send() {
    forwarder::endpoints::create_and_send(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deploy_contract() {
    forwarder::endpoints::deploy_contract(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deploy_two_contracts() {
    forwarder::endpoints::deploy_two_contracts(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn deploy_vault_from_source() {
    forwarder::endpoints::deploy_vault_from_source(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn echo_arguments_sync() {
    forwarder::endpoints::echo_arguments_sync(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn echo_arguments_sync_range() {
    forwarder::endpoints::echo_arguments_sync_range(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn echo_arguments_sync_twice() {
    forwarder::endpoints::echo_arguments_sync_twice(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_async_accept_funds() {
    forwarder::endpoints::forward_async_accept_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_async_accept_funds_half_payment() {
    forwarder::endpoints::forward_async_accept_funds_half_payment(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_async_accept_funds_with_fees() {
    forwarder::endpoints::forward_async_accept_funds_with_fees(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_async_retrieve_funds() {
    forwarder::endpoints::forward_async_retrieve_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds() {
    forwarder::endpoints::forward_sync_accept_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_multi_transfer() {
    forwarder::endpoints::forward_sync_accept_funds_multi_transfer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_then_read() {
    forwarder::endpoints::forward_sync_accept_funds_then_read(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_sync_accept_funds_with_fees() {
    forwarder::endpoints::forward_sync_accept_funds_with_fees(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_sync_retrieve_funds() {
    forwarder::endpoints::forward_sync_retrieve_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds() {
    forwarder::endpoints::forward_transf_exec_accept_funds(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_multi_transfer() {
    forwarder::endpoints::forward_transf_exec_accept_funds_multi_transfer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_return_values() {
    forwarder::endpoints::forward_transf_exec_accept_funds_return_values(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_transf_exec_accept_funds_twice() {
    forwarder::endpoints::forward_transf_exec_accept_funds_twice(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn forward_transf_execu_accept_funds_with_fees() {
    forwarder::endpoints::forward_transf_execu_accept_funds_with_fees(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getCurrentNftNonce() {
    forwarder::endpoints::getCurrentNftNonce(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn getFungibleEsdtBalance() {
    forwarder::endpoints::getFungibleEsdtBalance(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn get_nft_balance() {
    forwarder::endpoints::get_nft_balance(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn issue_fungible_token() {
    forwarder::endpoints::issue_fungible_token(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn lastErrorMessage() {
    forwarder::endpoints::lastErrorMessage(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn lastIssuedToken() {
    forwarder::endpoints::lastIssuedToken(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn local_burn() {
    forwarder::endpoints::local_burn(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn local_mint() {
    forwarder::endpoints::local_mint(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn multi_transfer_via_async() {
    forwarder::endpoints::multi_transfer_via_async(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn nft_add_quantity() {
    forwarder::endpoints::nft_add_quantity(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn nft_burn() {
    forwarder::endpoints::nft_burn(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn nft_create() {
    forwarder::endpoints::nft_create(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn nft_decode_complex_attributes() {
    forwarder::endpoints::nft_decode_complex_attributes(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn nft_issue() {
    forwarder::endpoints::nft_issue(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_egld() {
    forwarder::endpoints::send_egld(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_esdt() {
    forwarder::endpoints::send_esdt(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_esdt_direct_multi_transfer() {
    forwarder::endpoints::send_esdt_direct_multi_transfer(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_esdt_twice() {
    forwarder::endpoints::send_esdt_twice(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_esdt_with_fees() {
    forwarder::endpoints::send_esdt_with_fees(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn send_funds_twice() {
    forwarder::endpoints::send_funds_twice(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn setLocalRoles() {
    forwarder::endpoints::setLocalRoles(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn sft_issue() {
    forwarder::endpoints::sft_issue(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn transfer_nft_and_execute() {
    forwarder::endpoints::transfer_nft_and_execute(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn transfer_nft_via_async_call() {
    forwarder::endpoints::transfer_nft_via_async_call(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn unsetLocalRoles() {
    forwarder::endpoints::unsetLocalRoles(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn upgradeVault() {
    forwarder::endpoints::upgradeVault(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn upgrade_vault_from_source() {
    forwarder::endpoints::upgrade_vault_from_source(elrond_wasm_node::arwen_api());
}
