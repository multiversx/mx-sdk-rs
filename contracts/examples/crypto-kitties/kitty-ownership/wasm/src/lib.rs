////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_ownership::endpoints::init(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_ownership::endpoints::callBack(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn allowAuctioning() {
    kitty_ownership::endpoints::allowAuctioning(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn approve() {
    kitty_ownership::endpoints::approve(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn approveSiring() {
    kitty_ownership::endpoints::approveSiring(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn approveSiringAndReturnKitty() {
    kitty_ownership::endpoints::approveSiringAndReturnKitty(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn balanceOf() {
    kitty_ownership::endpoints::balanceOf(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn birthFee() {
    kitty_ownership::endpoints::birthFee(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn breedWith() {
    kitty_ownership::endpoints::breedWith(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn canBreedWith() {
    kitty_ownership::endpoints::canBreedWith(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn claim() {
    kitty_ownership::endpoints::claim(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn createGenZeroKitty() {
    kitty_ownership::endpoints::createGenZeroKitty(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn getKittyById() {
    kitty_ownership::endpoints::getKittyById(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn giveBirth() {
    kitty_ownership::endpoints::giveBirth(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn isPregnant() {
    kitty_ownership::endpoints::isPregnant(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn isReadyToBreed() {
    kitty_ownership::endpoints::isReadyToBreed(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn ownerOf() {
    kitty_ownership::endpoints::ownerOf(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn setGeneScienceContractAddress() {
    kitty_ownership::endpoints::setGeneScienceContractAddress(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn setKittyAuctionContractAddress() {
    kitty_ownership::endpoints::setKittyAuctionContractAddress(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn tokensOfOwner() {
    kitty_ownership::endpoints::tokensOfOwner(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn totalSupply() {
    kitty_ownership::endpoints::totalSupply(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer() {
    kitty_ownership::endpoints::transfer(elrond_wasm_node::vm_api());
}

#[no_mangle]
pub fn transfer_from() {
    kitty_ownership::endpoints::transfer_from(elrond_wasm_node::vm_api());
}
