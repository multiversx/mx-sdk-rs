// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback:                       1
// Total number of exported functions:   4

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    child
    (
        init => init
        issueWrappedEgld => issue_wrapped_egld
        getWrappedEgldTokenIdentifier => wrapped_egld_token_identifier
    )
}

multiversx_sc_wasm_adapter::async_callback! { child }
