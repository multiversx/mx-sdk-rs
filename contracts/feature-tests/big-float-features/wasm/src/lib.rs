// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           73
// Async Callback (empty):               1
// Total number of exported functions:  75

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    big_float_features
    (
        init => init
        new_from_parts_big_float => new_from_parts_big_float
        new_from_frac_big_float => new_from_frac_big_float
        new_from_sci_big_float => new_from_sci_big_float
        big_float_from_big_uint_1 => big_float_from_big_uint_1
        big_float_from_big_uint_2 => big_float_from_big_uint_2
        big_float_from_big_int_1 => big_float_from_big_int_1
        big_float_from_big_int_2 => big_float_from_big_int_2
        big_float_from_i64 => big_float_from_i64
        big_float_from_i32 => big_float_from_i32
        big_float_from_i16 => big_float_from_i16
        big_float_from_i8 => big_float_from_i8
        big_float_from_isize => big_float_from_isize
        big_float_from_man_buf => big_float_from_man_buf
        big_float_from_man_buf_ref => big_float_from_man_buf_ref
        sqrt_big_float => sqrt_big_float
        sqrt_big_float_ref => sqrt_big_float_ref
        pow_big_float => pow_big_float
        pow_big_float_ref => pow_big_float_ref
        big_float_zero => big_float_zero
        big_float_neg => big_float_neg
        add_big_float => add_big_float
        add_big_float_ref => add_big_float_ref
        sub_big_float => sub_big_float
        sub_big_float_ref => sub_big_float_ref
        mul_big_float => mul_big_float
        mul_big_float_ref => mul_big_float_ref
        div_big_float => div_big_float
        div_big_float_ref => div_big_float_ref
        add_assign_big_float => add_assign_big_float
        add_assign_big_float_ref => add_assign_big_float_ref
        sub_assign_big_float => sub_assign_big_float
        sub_assign_big_float_ref => sub_assign_big_float_ref
        mul_assign_big_float => mul_assign_big_float
        mul_assign_big_float_ref => mul_assign_big_float_ref
        div_assign_big_float => div_assign_big_float
        div_assign_big_float_ref => div_assign_big_float_ref
        new_from_parts_big_float_wrapped => new_from_parts_big_float_wrapped
        new_from_frac_big_float_wrapped => new_from_frac_big_float_wrapped
        new_from_sci_big_float_wrapped => new_from_sci_big_float_wrapped
        big_float_from_big_int_1_wrapped => big_float_from_big_int_1_wrapped
        big_float_from_big_int_2_wrapped => big_float_from_big_int_2_wrapped
        big_float_from_big_uint_1_wrapped => big_float_from_big_uint_1_wrapped
        big_float_from_big_uint_2_wrapped => big_float_from_big_uint_2_wrapped
        big_float_from_i64_wrapped => big_float_from_i64_wrapped
        big_float_from_i32_wrapped => big_float_from_i32_wrapped
        big_float_from_i16_wrapped => big_float_from_i16_wrapped
        big_float_from_i8_wrapped => big_float_from_i8_wrapped
        big_float_from_isize_wrapped => big_float_from_isize_wrapped
        sqrt_big_float_wrapped => sqrt_big_float_wrapped
        sqrt_big_float_ref_wrapped => sqrt_big_float_ref_wrapped
        pow_big_float_wrapped => pow_big_float_wrapped
        pow_big_float_ref_wrapped => pow_big_float_ref_wrapped
        big_float_zero_wrapped => big_float_zero_wrapped
        big_float_neg_wrapped => big_float_neg_wrapped
        ln_big_float_ref => ln_big_float_ref
        ln_big_float_precision_9 => ln_big_float_precision_9
        ln_big_float_any_precision => ln_big_float_any_precision
        add_big_float_wrapped => add_big_float_wrapped
        add_big_float_ref_wrapped => add_big_float_ref_wrapped
        sub_big_float_wrapped => sub_big_float_wrapped
        sub_big_float_ref_wrapped => sub_big_float_ref_wrapped
        mul_big_float_wrapped => mul_big_float_wrapped
        mul_big_float_ref_wrapped => mul_big_float_ref_wrapped
        div_big_float_wrapped => div_big_float_wrapped
        div_big_float_ref_wrapped => div_big_float_ref_wrapped
        add_assign_big_float_wrapped => add_assign_big_float_wrapped
        add_assign_big_float_ref_wrapped => add_assign_big_float_ref_wrapped
        sub_assign_big_float_wrapped => sub_assign_big_float_wrapped
        sub_assign_big_float_ref_wrapped => sub_assign_big_float_ref_wrapped
        mul_assign_big_float_wrapped => mul_assign_big_float_wrapped
        mul_assign_big_float_ref_wrapped => mul_assign_big_float_ref_wrapped
        div_assign_big_float_wrapped => div_assign_big_float_wrapped
        div_assign_big_float_ref_wrapped => div_assign_big_float_ref_wrapped
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
