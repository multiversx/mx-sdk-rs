// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    first_contract
    (
        transferToSecondContractFull
        transferToSecondContractHalf
        transferToSecondContractRejected
        transferToSecondContractRejectedWithTransferAndExecute
        transferToSecondContractFullWithTransferAndExecute
        getesdtTokenName
        getSecondContractAddress
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
