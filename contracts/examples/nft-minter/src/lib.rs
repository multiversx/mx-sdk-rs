#![no_std]

elrond_wasm::imports!();

mod nft_module;

#[elrond_wasm::derive::contract]
pub trait NftMinter: nft_module::NftModule {
    #[init]
    fn init(&self) {}
}
