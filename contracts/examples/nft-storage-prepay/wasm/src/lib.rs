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
    nft_storage_prepay
    (
        setCostPerByte
        reserveFunds
        claim
        depositPaymentForStorage
        withdraw
        getCostForSize
        getDepositAmount
        getCostPerByte
    )
}

mx_sc_wasm_adapter::wasm_empty_callback! {}
