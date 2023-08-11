// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            4
// Async Callback:                       1
// Total number of exported functions:   6

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    fractional_nfts
    (
        init => init
        claimRoyaltiesFromMarketplace => claim_royalties_from_marketplace
        fractionalizeNFT => fractionalize_nft
        unFractionalizeNFT => unfractionalize_nft
        getFractionalToken => fractional_token
    )
}

multiversx_sc_wasm_adapter::async_callback! { fractional_nfts }
