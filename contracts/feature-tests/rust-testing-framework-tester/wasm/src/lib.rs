// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           26
// Async Callback:                       1
// Total number of exported functions:  28

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!(static64k);
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    rust_testing_framework_tester
    (
        init => init
        sum => sum
        sum_sc_result => sum_sc_result
        get_caller_legacy => get_caller_legacy
        get_egld_balance => get_egld_balance
        get_esdt_balance => get_esdt_balance
        receive_egld => receive_egld
        receive_egld_half => receive_egld_half
        receive_esdt => receive_esdt
        reject_payment => reject_payment
        receive_esdt_half => receive_esdt_half
        receive_multi_esdt => receive_multi_esdt
        send_nft => send_nft
        mint_esdt => mint_esdt
        burn_esdt => burn_esdt
        create_nft => create_nft
        get_block_epoch => get_block_epoch
        get_block_nonce => get_block_nonce
        get_block_timestamp => get_block_timestamp
        get_random_buffer_once => get_random_buffer_once
        get_random_buffer_twice => get_random_buffer_twice
        call_other_contract_execute_on_dest => call_other_contract_execute_on_dest
        call_other_contract_add_async_call => call_other_contract_add_async_call
        getTotalValue => get_total_value
        execute_on_dest_add_value => execute_on_dest_add_value
        addValue => add
        panic => panic
    )
}

multiversx_sc_wasm_adapter::async_callback! { rust_testing_framework_tester }
