#[test]
fn big_int_to_i64_go() {
    elrond_wasm_debug::mandos_go("mandos/big_int_to_i64.scen.json");
}

#[test]
fn big_num_conversions_go() {
    elrond_wasm_debug::mandos_go("mandos/big_num_conversions.scen.json");
}

#[test]
fn big_uint_sqrt_go() {
    elrond_wasm_debug::mandos_go("mandos/big_uint_sqrt.scen.json");
}

#[test]
fn big_uint_to_u64_go() {
    elrond_wasm_debug::mandos_go("mandos/big_uint_to_u64.scen.json");
}

#[test]
fn block_info_go() {
    elrond_wasm_debug::mandos_go("mandos/block_info.scen.json");
}

#[test]
fn codec_err_go() {
    elrond_wasm_debug::mandos_go("mandos/codec_err.scen.json");
}

#[test]
fn count_ones_go() {
    elrond_wasm_debug::mandos_go("mandos/count_ones.scen.json");
}

#[test]
fn crypto_elliptic_curves_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_elliptic_curves.scen.json");
}

#[test]
fn crypto_keccak256_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_keccak256.scen.json");
}

#[test]
fn crypto_keccak256_legacy_managed_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_keccak256_legacy_managed.scen.json");
}

#[test]
fn crypto_ripemd160_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_ripemd160.scen.json");
}

#[test]
fn crypto_sha256_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_sha256.scen.json");
}

#[test]
fn crypto_verify_bls_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_bls.scen.json");
}

#[test]
fn crypto_verify_ed25519_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_ed25519.scen.json");
}

#[test]
fn crypto_verify_secp256k1_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_secp256k1.scen.json");
}

#[test]
fn echo_array_u8_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_array_u8.scen.json");
}

#[test]
fn echo_arrayvec_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_arrayvec.scen.json");
}

#[test]
fn echo_big_int_nested_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_big_int_nested.scen.json");
}

#[test]
fn echo_big_int_top_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_big_int_top.scen.json");
}

#[test]
fn echo_big_uint_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_big_uint.scen.json");
}

#[test]
fn echo_i32_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_i32.scen.json");
}

#[test]
fn echo_i64_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_i64.scen.json");
}

#[test]
fn echo_ignore_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_ignore.scen.json");
}

#[test]
fn echo_managed_async_result_empty_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_managed_async_result_empty.scen.json");
}

#[test]
fn echo_managed_bytes_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_managed_bytes.scen.json");
}

#[test]
fn echo_managed_vec_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_managed_vec.scen.json");
}

#[test]
fn echo_multi_value_tuples_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_multi_value_tuples.scen.json");
}

#[test]
fn echo_nothing_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_nothing.scen.json");
}

#[test]
fn echo_tuple_into_multiresult_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_tuple_into_multiresult.scen.json");
}

#[test]
fn echo_u64_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_u64.scen.json");
}

#[test]
fn echo_usize_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_usize.scen.json");
}

#[test]
fn echo_varargs_managed_eager_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_varargs_managed_eager.scen.json");
}

#[test]
fn echo_varargs_managed_sum_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_varargs_managed_sum.scen.json");
}

#[test]
fn echo_varargs_u32_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_varargs_u32.scen.json");
}

#[test]
fn events_go() {
    elrond_wasm_debug::mandos_go("mandos/events.scen.json");
}

#[test]
fn get_caller_go() {
    elrond_wasm_debug::mandos_go("mandos/get_caller.scen.json");
}

#[test]
fn get_cumulated_validator_rewards_go() {
    elrond_wasm_debug::mandos_go("mandos/get_cumulated_validator_rewards.scen.json");
}

#[test]
fn managed_address_array_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_address_array.scen.json");
}

#[test]
fn managed_address_managed_buffer_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_address_managed_buffer.scen.json");
}

#[test]
fn managed_buffer_concat_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_concat.scen.json");
}

#[test]
fn managed_buffer_copy_slice_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_copy_slice.scen.json");
}

#[test]
fn managed_buffer_eq_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_eq.scen.json");
}

#[test]
fn managed_buffer_set_random_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_set_random.scen.json");
}

#[test]
fn managed_vec_address_push_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_vec_address_push.scen.json");
}

#[test]
fn managed_vec_biguint_push_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_vec_biguint_push.scen.json");
}

#[test]
fn only_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/only_owner.scen.json");
}

#[test]
fn out_of_gas_go() {
    elrond_wasm_debug::mandos_go("mandos/out_of_gas.scen.json");
}

#[test]
fn panic_go() {
    elrond_wasm_debug::mandos_go("mandos/panic.scen.json");
}

#[test]
fn return_codes_go() {
    elrond_wasm_debug::mandos_go("mandos/return_codes.scen.json");
}

#[test]
fn sc_properties_go() {
    elrond_wasm_debug::mandos_go("mandos/sc_properties.scen.json");
}

#[test]
fn storage_big_int_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_big_int.scen.json");
}

#[test]
fn storage_big_uint_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_big_uint.scen.json");
}

#[test]
fn storage_bool_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_bool.scen.json");
}

#[test]
fn storage_clear_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_clear.scen.json");
}

#[test]
fn storage_i64_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_i64.scen.json");
}

#[test]
fn storage_i64_bad_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_i64_bad.scen.json");
}

#[test]
fn storage_load_from_address_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_load_from_address.scen.json");
}

#[test]
fn storage_managed_address_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_managed_address.scen.json");
}

#[test]
fn storage_map1_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_map1.scen.json");
}

#[test]
fn storage_map2_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_map2.scen.json");
}

#[test]
fn storage_map3_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_map3.scen.json");
}

#[test]
fn storage_mapper_fungible_token_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_fungible_token.scen.json");
}

#[test]
fn storage_mapper_linked_list_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_linked_list.scen.json");
}

#[test]
fn storage_mapper_map_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_map.scen.json");
}

#[test]
fn storage_mapper_map_storage_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_map_storage.scen.json");
}

#[test]
fn storage_mapper_non_fungible_token_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_non_fungible_token.scen.json");
}

#[test]
fn storage_mapper_queue_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_queue.scen.json");
}

#[test]
fn storage_mapper_set_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_set.scen.json");
}

#[test]
fn storage_mapper_single_value_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_single_value.scen.json");
}

#[test]
fn storage_mapper_token_attributes_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_token_attributes.scen.json");
}

#[test]
fn storage_mapper_vec_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_vec.scen.json");
}

#[test]
fn storage_mapper_whitelist_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_mapper_whitelist.scen.json");
}

#[test]
fn storage_opt_managed_addr_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_opt_managed_addr.scen.json");
}

#[test]
fn storage_reserved_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_reserved.scen.json");
}

#[test]
fn storage_u64_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_u64.scen.json");
}

#[test]
fn storage_u64_bad_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_u64_bad.scen.json");
}

#[test]
fn storage_usize_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_usize.scen.json");
}

#[test]
fn storage_usize_bad_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_usize_bad.scen.json");
}

#[test]
fn struct_eq_go() {
    elrond_wasm_debug::mandos_go("mandos/struct_eq.scen.json");
}
