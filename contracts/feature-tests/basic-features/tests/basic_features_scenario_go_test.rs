#[test]
fn big_int_from_i64_go() {
    multiversx_sc_scenario::run_go("scenarios/big_int_from_i64.scen.json");
}

#[test]
fn big_int_to_i64_go() {
    multiversx_sc_scenario::run_go("scenarios/big_int_to_i64.scen.json");
}

#[test]
fn big_num_conversions_go() {
    multiversx_sc_scenario::run_go("scenarios/big_num_conversions.scen.json");
}

#[test]
fn big_uint_eq_u64_go() {
    multiversx_sc_scenario::run_go("scenarios/big_uint_eq_u64.scen.json");
}

#[test]
fn big_uint_sqrt_go() {
    multiversx_sc_scenario::run_go("scenarios/big_uint_sqrt.scen.json");
}

#[test]
fn big_uint_from_u64_go() {
    multiversx_sc_scenario::run_go("scenarios/big_uint_from_u64.scen.json");
}

#[test]
fn big_uint_to_u64_go() {
    multiversx_sc_scenario::run_go("scenarios/big_uint_to_u64.scen.json");
}

#[test]
fn block_info_go() {
    multiversx_sc_scenario::run_go("scenarios/block_info.scen.json");
}

#[test]
fn codec_err_go() {
    multiversx_sc_scenario::run_go("scenarios/codec_err.scen.json");
}

#[test]
fn count_ones_go() {
    multiversx_sc_scenario::run_go("scenarios/count_ones.scen.json");
}

#[test]
fn crypto_elliptic_curves_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_elliptic_curves.scen.json");
}

#[test]
fn crypto_keccak256_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_keccak256.scen.json");
}

#[test]
fn crypto_keccak256_legacy_managed_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_keccak256_legacy_managed.scen.json");
}

#[test]
fn crypto_ripemd160_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_ripemd160.scen.json");
}

#[test]
fn crypto_sha256_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_sha256.scen.json");
}

#[test]
fn crypto_verify_bls_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_verify_bls.scen.json");
}

#[test]
fn crypto_verify_ed25519_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_verify_ed25519.scen.json");
}

#[test]
fn crypto_verify_secp256k1_go() {
    multiversx_sc_scenario::run_go("scenarios/crypto_verify_secp256k1.scen.json");
}

#[test]
fn echo_array_u8_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_array_u8.scen.json");
}

#[test]
fn echo_arrayvec_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_arrayvec.scen.json");
}

#[test]
fn echo_big_int_nested_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_big_int_nested.scen.json");
}

#[test]
fn echo_big_int_top_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_big_int_top.scen.json");
}

#[test]
fn echo_big_uint_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_big_uint.scen.json");
}

#[test]
fn echo_i32_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_i32.scen.json");
}

#[test]
fn echo_i64_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_i64.scen.json");
}

#[test]
fn echo_ignore_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_ignore.scen.json");
}

#[test]
fn echo_managed_async_result_empty_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_managed_async_result_empty.scen.json");
}

#[test]
fn echo_managed_bytes_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_managed_bytes.scen.json");
}

#[test]
fn echo_managed_vec_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_managed_vec.scen.json");
}

#[test]
fn echo_multi_value_tuples_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_multi_value_tuples.scen.json");
}

#[test]
fn echo_nothing_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_nothing.scen.json");
}

#[test]
fn echo_tuple_into_multiresult_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_tuple_into_multiresult.scen.json");
}

#[test]
fn echo_u64_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_u64.scen.json");
}

#[test]
fn echo_usize_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_usize.scen.json");
}

#[test]
fn echo_varargs_managed_eager_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_varargs_managed_eager.scen.json");
}

#[test]
fn echo_varargs_managed_sum_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_varargs_managed_sum.scen.json");
}

#[test]
fn echo_varargs_u32_go() {
    multiversx_sc_scenario::run_go("scenarios/echo_varargs_u32.scen.json");
}

#[test]
fn events_go() {
    multiversx_sc_scenario::run_go("scenarios/events.scen.json");
}

#[test]
fn get_caller_go() {
    multiversx_sc_scenario::run_go("scenarios/get_caller.scen.json");
}

#[test]
fn get_cumulated_validator_rewards_go() {
    multiversx_sc_scenario::run_go("scenarios/get_cumulated_validator_rewards.scen.json");
}

#[test]
fn managed_address_array_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_address_array.scen.json");
}

#[test]
fn managed_address_managed_buffer_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_address_managed_buffer.scen.json");
}

#[test]
fn managed_buffer_concat_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_buffer_concat.scen.json");
}

#[test]
fn managed_buffer_copy_slice_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_buffer_copy_slice.scen.json");
}

#[test]
fn managed_buffer_eq_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_buffer_eq.scen.json");
}

#[test]
fn managed_buffer_set_random_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_buffer_set_random.scen.json");
}

#[test]
fn managed_vec_address_push_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_vec_address_push.scen.json");
}

#[test]
fn managed_vec_biguint_push_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_vec_biguint_push.scen.json");
}

#[test]
fn managed_vec_array_push_go() {
    multiversx_sc_scenario::run_go("scenarios/managed_vec_array_push.scen.json");
}

#[test]
fn only_owner_go() {
    multiversx_sc_scenario::run_go("scenarios/only_owner.scen.json");
}

#[test]
fn only_user_account_go() {
    multiversx_sc_scenario::run_go("scenarios/only_user_account.scen.json");
}

#[test]
fn out_of_gas_go() {
    multiversx_sc_scenario::run_go("scenarios/out_of_gas.scen.json");
}

#[test]
fn panic_go() {
    multiversx_sc_scenario::run_go("scenarios/panic.scen.json");
}

#[test]
fn return_codes_go() {
    multiversx_sc_scenario::run_go("scenarios/return_codes.scen.json");
}

#[test]
fn sc_properties_go() {
    multiversx_sc_scenario::run_go("scenarios/sc_properties.scen.json");
}

#[test]
fn storage_raw_api_features_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_raw_api_features.scen.json");
}

#[test]
fn storage_big_int_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_big_int.scen.json");
}

#[test]
fn storage_big_uint_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_big_uint.scen.json");
}

#[test]
fn storage_bool_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_bool.scen.json");
}

#[test]
fn storage_clear_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_clear.scen.json");
}

#[test]
fn storage_i64_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_i64.scen.json");
}

#[test]
fn storage_i64_bad_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_i64_bad.scen.json");
}

#[test]
fn storage_load_from_address_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_load_from_address.scen.json");
}

#[test]
fn storage_managed_address_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_managed_address.scen.json");
}

#[test]
fn storage_map1_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_map1.scen.json");
}

#[test]
fn storage_map2_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_map2.scen.json");
}

#[test]
fn storage_map3_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_map3.scen.json");
}

#[test]
fn storage_mapper_fungible_token_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_fungible_token.scen.json");
}

#[test]
fn storage_mapper_linked_list_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_linked_list.scen.json");
}

#[test]
fn storage_mapper_map_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_map.scen.json");
}

#[test]
fn storage_mapper_map_storage_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_map_storage.scen.json");
}

#[test]
fn storage_mapper_non_fungible_token_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_non_fungible_token.scen.json");
}

#[test]
fn storage_mapper_queue_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_queue.scen.json");
}

#[test]
fn storage_mapper_set_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_set.scen.json");
}

#[test]
fn storage_mapper_single_value_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_single_value.scen.json");
}

#[test]
fn storage_mapper_token_attributes_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_token_attributes.scen.json");
}

#[test]
fn storage_mapper_vec_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_vec.scen.json");
}

#[test]
fn storage_mapper_whitelist_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_whitelist.scen.json");
}

#[test]
fn storage_opt_managed_addr_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_opt_managed_addr.scen.json");
}

#[test]
fn storage_reserved_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_reserved.scen.json");
}

#[test]
fn storage_u64_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_u64.scen.json");
}

#[test]
fn storage_u64_bad_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_u64_bad.scen.json");
}

#[test]
fn storage_usize_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_usize.scen.json");
}

#[test]
fn storage_usize_bad_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_usize_bad.scen.json");
}

#[test]
fn struct_eq_go() {
    multiversx_sc_scenario::run_go("scenarios/struct_eq.scen.json");
}

#[test]
fn storage_mapper_unique_id_go() {
    multiversx_sc_scenario::run_go("scenarios/storage_mapper_unique_id.scen.json");
}
