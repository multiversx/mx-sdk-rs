// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            1
// Async Callback:                       1
// Total number of exported functions:   3

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    recursive_caller
    (
        init => init
        recursive_send_funds => recursive_send_funds
    )
}

multiversx_sc_wasm_adapter::async_callback! { recursive_caller }
