// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::external_view_init! {}

multiversx_sc_wasm_adapter::external_view_endpoints! {
    abi_tester
    (
        external_view => external_view
        payable_any_token => payable_any_token
        label_a => label_a
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
