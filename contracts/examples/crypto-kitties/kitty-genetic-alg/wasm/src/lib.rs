////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]
#![allow(non_snake_case)]

pub use elrond_wasm_output;

#[no_mangle]
pub fn init() {
    kitty_genetic_alg::endpoints::init(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn callBack() {
    kitty_genetic_alg::endpoints::callBack(elrond_wasm_node::arwen_api());
}

#[no_mangle]
pub fn generateKittyGenes() {
    kitty_genetic_alg::endpoints::generateKittyGenes(elrond_wasm_node::arwen_api());
}
