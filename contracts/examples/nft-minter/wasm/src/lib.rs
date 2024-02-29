// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback:                       1
// Total number of exported functions:   8

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nft_minter
    (
        init => init
        createNft => create_nft
        claimRoyaltiesFromMarketplace => claim_royalties_from_marketplace
        issueToken => issue_token
        setLocalRoles => set_local_roles
        buyNft => buy_nft
        getNftPrice => get_nft_price
    )
}

multiversx_sc_wasm_adapter::async_callback! { nft_minter }
