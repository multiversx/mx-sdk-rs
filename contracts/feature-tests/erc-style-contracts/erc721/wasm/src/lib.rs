// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

mx_sc_wasm_adapter::wasm_endpoints! {
    erc721
    (
        mint
        approve
        revoke
        transfer
        totalMinted
        tokenOwner
        tokenCount
        approval
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
