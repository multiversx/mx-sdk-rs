use elrond_wasm::*;
use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(
        "file:output/basic-features.wasm",
        Box::new(|context| Box::new(basic_features::contract_obj(context))),
    );
    blockchain
}

#[test]
fn big_int_to_i64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_int_to_i64.scen.json", world());
}

#[test]
fn echo_ignore_go() {
    elrond_wasm_debug::mandos_rs("mandos/echo_ignore.scen.json", world());
}

#[test]
fn big_num_conversions_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_num_conversions.scen.json", world());
}

#[test]
fn big_uint_sqrt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_uint_sqrt.scen.json", world());
}

#[test]
fn big_uint_to_u64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/big_uint_to_u64.scen.json", world());
}

#[test]
fn block_info_rs() {
    elrond_wasm_debug::mandos_rs("mandos/block_info.scen.json", world());
}

#[test]
fn boxed_bytes_zeros_rs() {
    elrond_wasm_debug::mandos_rs("mandos/boxed_bytes_zeros.scen.json", world());
}

#[test]
fn codec_err_rs() {
    elrond_wasm_debug::mandos_rs("mandos/codec_err.scen.json", world());
}

#[test]
fn count_ones_rs() {
    elrond_wasm_debug::mandos_rs("mandos/count_ones.scen.json", world());
}

// #[test]
// fn crypto_elliptic_curves_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_elliptic_curves.scen.json", world());
// }

#[test]
fn crypto_keccak256_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crypto_keccak256.scen.json", world());
}

// #[test]
// fn crypto_ripemd160_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_ripemd160.scen.json", world());
// }

#[test]
fn crypto_sha256_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crypto_sha256.scen.json", world());
}

// #[test]
// fn crypto_verify_funcs_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_verify_funcs.scen.json", world());
// }

#[test]
fn echo_array_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_array_u8.scen.json", world());
}

#[test]
fn echo_arrayvec_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_arrayvec.scen.json", world());
}

#[test]
fn echo_async_result_empty_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_async_result_empty.scen.json", world());
}

#[test]
fn echo_async_result_empty_managed_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_async_result_empty_managed.scen.json", world());
}

#[test]
fn echo_big_int_nested_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_big_int_nested.scen.json", world());
}

#[test]
fn echo_big_int_top_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_big_int_top.scen.json", world());
}

#[test]
fn echo_big_uint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_big_uint.scen.json", world());
}

#[test]
fn echo_boxed_bytes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_boxed_bytes.scen.json", world());
}

#[test]
fn echo_i32_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_i32.scen.json", world());
}

#[test]
fn echo_i64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_i64.scen.json", world());
}

#[test]
fn echo_managed_bytes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_managed_bytes.scen.json", world());
}

#[test]
fn echo_managed_vec_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_managed_vec.scen.json", world());
}

#[test]
fn echo_nothing_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_nothing.scen.json", world());
}

#[test]
fn echo_ser_ex_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_ser_ex_1.scen.json", world());
}

#[test]
fn echo_slice_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_slice_u8.scen.json", world());
}

#[test]
fn echo_str_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_str.scen.json", world());
}

#[test]
fn echo_str_box_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_str_box.scen.json", world());
}

#[test]
fn echo_string_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_string.scen.json", world());
}

#[test]
fn echo_tuple_into_multiresult_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_tuple_into_multiresult.scen.json", world());
}

#[test]
fn echo_u64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_u64.scen.json", world());
}

#[test]
fn echo_usize_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_usize.scen.json", world());
}

#[test]
fn echo_varargs_managed_eager_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_varargs_managed_eager.scen.json", world());
}

#[test]
fn echo_varargs_managed_sum_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_varargs_managed_sum.scen.json", world());
}

#[test]
fn echo_varags_tuples_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_varags_tuples.scen.json", world());
}

#[test]
fn echo_varargs_u32_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_varargs_u32.scen.json", world());
}

#[test]
fn echo_vec_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_vec_u8.scen.json", world());
}

#[test]
fn events_rs() {
    elrond_wasm_debug::mandos_rs("mandos/events.scen.json", world());
}

#[test]
fn events_legacy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/events_legacy.scen.json", world());
}

#[test]
fn get_caller_rs() {
    elrond_wasm_debug::mandos_rs("mandos/get_caller.scen.json", world());
}

#[test]
fn get_cumulated_validator_rewards_rs() {
    elrond_wasm_debug::mandos_rs("mandos/get_cumulated_validator_rewards.scen.json", world());
}

// TODO: uncomment after implemented the full ESDT format in mandos-rs
// #[test]
// fn get_esdt_local_roles_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/get_esdt_local_roles.scen.json",
// 		world(),
// 	);
// }

#[test]
fn managed_address_array_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_address_array.scen.json", world());
}

#[test]
fn managed_address_managed_buffer_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_address_managed_buffer.scen.json", world());
}

#[test]
fn managed_buffer_concat_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_concat_1.scen.json", world());
}

#[test]
fn managed_buffer_concat_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_concat_2.scen.json", world());
}

#[test]
fn managed_buffer_eq_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_eq.scen.json", world());
}

#[test]
fn managed_buffer_overwrite_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_overwrite.scen.json", world());
}

#[test]
fn managed_buffer_slice_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_slice_1.scen.json", world());
}

#[test]
fn managed_buffer_slice_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_slice_2.scen.json", world());
}

#[test]
fn managed_vec_address_push_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_vec_address_push.scen.json", world());
}

#[test]
fn managed_vec_biguint_push_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_vec_biguint_push.scen.json", world());
}

#[test]
fn only_owner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/only_owner.scen.json", world());
}

// Will never run in mandos-rs.
// #[test]
// fn out_of_gas_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/out_of_gas.scen.json", world());
// }

#[test]
fn panic_rs() {
    elrond_wasm_debug::mandos_rs("mandos/panic.scen.json", world());
}

#[test]
fn return_codes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/return_codes.scen.json", world());
}

#[test]
fn sc_properties_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sc_properties.scen.json", world());
}

#[test]
fn sc_result_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sc_result.scen.json", world());
}

#[test]
fn storage_addr_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_addr.scen.json", world());
}

#[test]
fn storage_big_int_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_big_int.scen.json", world());
}

#[test]
fn storage_big_uint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_big_uint.scen.json", world());
}

#[test]
fn storage_bool_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_bool.scen.json", world());
}

#[test]
fn storage_clear_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_clear.scen.json", world());
}

#[test]
fn storage_i64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_i64.scen.json", world());
}

#[test]
fn storage_i64_bad_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_i64_bad.scen.json", world());
}

#[test]
fn storage_map1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_map1.scen.json", world());
}

#[test]
fn storage_map2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_map2.scen.json", world());
}

#[test]
fn storage_map3_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_map3.scen.json", world());
}

#[test]
fn storage_mapper_linked_list_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_linked_list.scen.json", world());
}

#[test]
fn storage_mapper_queue_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_queue.scen.json", world());
}

#[test]
fn storage_mapper_map_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_map.scen.json", world());
}

#[test]
fn storage_mapper_map_storage_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_map_storage.scen.json", world());
}

#[test]
fn storage_mapper_set_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_set.scen.json", world());
}

#[test]
fn storage_mapper_single_value_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_single_value.scen.json", world());
}

#[test]
fn storage_mapper_token_attributes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_token_attributes.scen.json", world());
}

#[test]
fn storage_mapper_vec_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_mapper_vec.scen.json", world());
}

#[test]
fn storage_opt_addr_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_opt_addr.scen.json", world());
}

#[test]
fn storage_reserved_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_reserved.scen.json", world());
}

#[test]
fn storage_u64_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_u64.scen.json", world());
}

#[test]
fn storage_u64_bad_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_u64_bad.scen.json", world());
}

#[test]
fn storage_usize_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_usize.scen.json", world());
}

#[test]
fn storage_usize_bad_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_usize_bad.scen.json", world());
}

#[test]
fn storage_vec_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_vec_u8.scen.json", world());
}
