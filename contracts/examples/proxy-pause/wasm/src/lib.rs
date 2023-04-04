// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![feature(alloc_error_handler, lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    proxy_pause
    (
        init => init
        addContracts => add_contracts
        removeContracts => remove_contracts
        addOwners => add_owners
        removeOwners => remove_owners
        pause => pause
        unpause => unpause
        owners => owners
        contracts => contracts
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
