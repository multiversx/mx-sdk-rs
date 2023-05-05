// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    rust_snippets_generator_test
    (
        no_arg_no_result_endpoint
        no_arg_one_result_endpoint
        one_arg_no_result_endpoint
        one_arg_one_result_endpoint
        multi_result
        nested_result
        custom_struct
        optional_type
        option_type
        esdt_token_payment
        egld_or_esdt_payment
        egld_only_endpoint
        payable_endpoint
        managed_buffer
        multi_value_2
        multi_value_4
        complex_multi_values
        view_func
        view_custom_type
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
