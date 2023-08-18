use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/basic-features");

    blockchain.register_contract(
        "file:output/basic-features.wasm",
        basic_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:../esdt-system-sc-mock/output/esdt-system-sc-mock.wasm",
        esdt_system_sc_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn big_int_from_i_64_rs() {
    world().run("scenarios/big_int_from_i64.scen.json");
}

#[test]
fn big_int_to_i_64_rs() {
    world().run("scenarios/big_int_to_i64.scen.json");
}

#[test]
fn big_num_conversions_rs() {
    world().run("scenarios/big_num_conversions.scen.json");
}

#[test]
fn big_uint_eq_u_64_rs() {
    world().run("scenarios/big_uint_eq_u64.scen.json");
}

#[test]
fn big_uint_from_u_64_rs() {
    world().run("scenarios/big_uint_from_u64.scen.json");
}

#[test]
fn big_uint_sqrt_rs() {
    world().run("scenarios/big_uint_sqrt.scen.json");
}

#[test]
fn big_uint_to_u_64_rs() {
    world().run("scenarios/big_uint_to_u64.scen.json");
}

#[test]
fn block_info_rs() {
    world().run("scenarios/block_info.scen.json");
}

#[test]
fn codec_err_rs() {
    world().run("scenarios/codec_err.scen.json");
}

#[test]
fn count_ones_rs() {
    world().run("scenarios/count_ones.scen.json");
}

#[test]
#[ignore]
fn crypto_elliptic_curves_rs() {
    world().run("scenarios/crypto_elliptic_curves.scen.json");
}

#[test]
fn crypto_keccak_256_rs() {
    world().run("scenarios/crypto_keccak256.scen.json");
}

#[test]
#[ignore]
fn crypto_ripemd_160_rs() {
    world().run("scenarios/crypto_ripemd160.scen.json");
}

#[test]
fn crypto_sha_256_rs() {
    world().run("scenarios/crypto_sha256.scen.json");
}

#[test]
#[ignore]
fn crypto_verify_bls_rs() {
    world().run("scenarios/crypto_verify_bls.scen.json");
}

#[test]
fn crypto_verify_ed_25519_rs() {
    world().run("scenarios/crypto_verify_ed25519.scen.json");
}

#[test]
#[ignore]
fn crypto_verify_secp_256_k_1_rs() {
    world().run("scenarios/crypto_verify_secp256k1.scen.json");
}

#[test]
fn echo_array_u_8_rs() {
    world().run("scenarios/echo_array_u8.scen.json");
}

#[test]
fn echo_arrayvec_rs() {
    world().run("scenarios/echo_arrayvec.scen.json");
}

#[test]
fn echo_big_int_nested_rs() {
    world().run("scenarios/echo_big_int_nested.scen.json");
}

#[test]
fn echo_big_int_top_rs() {
    world().run("scenarios/echo_big_int_top.scen.json");
}

#[test]
fn echo_big_uint_rs() {
    world().run("scenarios/echo_big_uint.scen.json");
}

#[test]
fn echo_i_32_rs() {
    world().run("scenarios/echo_i32.scen.json");
}

#[test]
fn echo_i_64_rs() {
    world().run("scenarios/echo_i64.scen.json");
}

#[test]
fn echo_ignore_rs() {
    world().run("scenarios/echo_ignore.scen.json");
}

#[test]
fn echo_managed_async_result_empty_rs() {
    world().run("scenarios/echo_managed_async_result_empty.scen.json");
}

#[test]
fn echo_managed_bytes_rs() {
    world().run("scenarios/echo_managed_bytes.scen.json");
}

#[test]
fn echo_managed_vec_rs() {
    world().run("scenarios/echo_managed_vec.scen.json");
}

#[test]
fn echo_multi_value_tuples_rs() {
    world().run("scenarios/echo_multi_value_tuples.scen.json");
}

#[test]
fn echo_nothing_rs() {
    world().run("scenarios/echo_nothing.scen.json");
}

#[test]
fn echo_tuple_into_multiresult_rs() {
    world().run("scenarios/echo_tuple_into_multiresult.scen.json");
}

#[test]
fn echo_u_64_rs() {
    world().run("scenarios/echo_u64.scen.json");
}

#[test]
fn echo_usize_rs() {
    world().run("scenarios/echo_usize.scen.json");
}

#[test]
fn echo_varargs_managed_eager_rs() {
    world().run("scenarios/echo_varargs_managed_eager.scen.json");
}

#[test]
fn echo_varargs_managed_sum_rs() {
    world().run("scenarios/echo_varargs_managed_sum.scen.json");
}

#[test]
fn echo_varargs_u_32_rs() {
    world().run("scenarios/echo_varargs_u32.scen.json");
}

#[test]
fn events_rs() {
    world().run("scenarios/events.scen.json");
}

#[test]
fn get_caller_rs() {
    world().run("scenarios/get_caller.scen.json");
}

#[test]
fn get_cumulated_validator_rewards_rs() {
    world().run("scenarios/get_cumulated_validator_rewards.scen.json");
}

#[test]
fn get_shard_of_address_rs() {
    world().run("scenarios/get_shard_of_address.scen.json");
}

#[test]
fn managed_address_array_rs() {
    world().run("scenarios/managed_address_array.scen.json");
}

#[test]
fn managed_address_managed_buffer_rs() {
    world().run("scenarios/managed_address_managed_buffer.scen.json");
}

#[test]
fn managed_buffer_concat_rs() {
    world().run("scenarios/managed_buffer_concat.scen.json");
}

#[test]
fn managed_buffer_copy_slice_rs() {
    world().run("scenarios/managed_buffer_copy_slice.scen.json");
}

#[test]
fn managed_buffer_eq_rs() {
    world().run("scenarios/managed_buffer_eq.scen.json");
}

#[test]
#[ignore]
fn managed_buffer_set_random_rs() {
    world().run("scenarios/managed_buffer_set_random.scen.json");
}

#[test]
fn managed_vec_address_push_rs() {
    world().run("scenarios/managed_vec_address_push.scen.json");
}

#[test]
fn managed_vec_array_push_rs() {
    world().run("scenarios/managed_vec_array_push.scen.json");
}

#[test]
fn managed_vec_biguint_push_rs() {
    world().run("scenarios/managed_vec_biguint_push.scen.json");
}

#[test]
fn only_owner_rs() {
    world().run("scenarios/only_owner.scen.json");
}

#[test]
fn only_user_account_rs() {
    world().run("scenarios/only_user_account.scen.json");
}

// Will never run in scenarios-rs.
#[test]
#[ignore]
fn out_of_gas_rs() {
    world().run("scenarios/out_of_gas.scen.json");
}

#[test]
fn panic_rs() {
    world().run("scenarios/panic.scen.json");
}

#[test]
fn return_codes_rs() {
    world().run("scenarios/return_codes.scen.json");
}

#[test]
fn sc_properties_rs() {
    world().run("scenarios/sc_properties.scen.json");
}

#[test]
fn storage_big_int_rs() {
    world().run("scenarios/storage_big_int.scen.json");
}

#[test]
fn storage_big_uint_rs() {
    world().run("scenarios/storage_big_uint.scen.json");
}

#[test]
fn storage_bool_rs() {
    world().run("scenarios/storage_bool.scen.json");
}

#[test]
fn storage_clear_rs() {
    world().run("scenarios/storage_clear.scen.json");
}

#[test]
fn storage_i_64_rs() {
    world().run("scenarios/storage_i64.scen.json");
}

#[test]
fn storage_i_64_bad_rs() {
    world().run("scenarios/storage_i64_bad.scen.json");
}

#[test]
fn storage_load_from_address_rs() {
    world().run("scenarios/storage_load_from_address.scen.json");
}

#[test]
fn storage_managed_address_rs() {
    world().run("scenarios/storage_managed_address.scen.json");
}

#[test]
fn storage_map_1_rs() {
    world().run("scenarios/storage_map1.scen.json");
}

#[test]
fn storage_map_2_rs() {
    world().run("scenarios/storage_map2.scen.json");
}

#[test]
fn storage_map_3_rs() {
    world().run("scenarios/storage_map3.scen.json");
}

#[test]
#[ignore]
fn storage_mapper_fungible_token_rs() {
    world().run("scenarios/storage_mapper_fungible_token.scen.json");
}

#[test]
fn storage_mapper_linked_list_rs() {
    world().run("scenarios/storage_mapper_linked_list.scen.json");
}

#[test]
fn storage_mapper_map_rs() {
    world().run("scenarios/storage_mapper_map.scen.json");
}

#[test]
fn storage_mapper_map_storage_rs() {
    world().run("scenarios/storage_mapper_map_storage.scen.json");
}

#[test]
#[ignore]
fn storage_mapper_non_fungible_token_rs() {
    world().run("scenarios/storage_mapper_non_fungible_token.scen.json");
}

#[test]
fn storage_mapper_queue_rs() {
    world().run("scenarios/storage_mapper_queue.scen.json");
}

#[test]
fn storage_mapper_set_rs() {
    world().run("scenarios/storage_mapper_set.scen.json");
}

#[test]
fn storage_mapper_single_value_rs() {
    world().run("scenarios/storage_mapper_single_value.scen.json");
}

#[test]
fn storage_mapper_token_attributes_rs() {
    world().run("scenarios/storage_mapper_token_attributes.scen.json");
}

#[test]
fn storage_mapper_unique_id_rs() {
    world().run("scenarios/storage_mapper_unique_id.scen.json");
}

#[test]
fn storage_mapper_vec_rs() {
    world().run("scenarios/storage_mapper_vec.scen.json");
}

#[test]
fn storage_mapper_whitelist_rs() {
    world().run("scenarios/storage_mapper_whitelist.scen.json");
}

#[test]
fn storage_opt_managed_addr_rs() {
    world().run("scenarios/storage_opt_managed_addr.scen.json");
}

#[test]
fn storage_raw_api_features_rs() {
    world().run("scenarios/storage_raw_api_features.scen.json");
}

#[test]
fn storage_reserved_rs() {
    world().run("scenarios/storage_reserved.scen.json");
}

#[test]
fn storage_u_64_rs() {
    world().run("scenarios/storage_u64.scen.json");
}

#[test]
fn storage_u_64_bad_rs() {
    world().run("scenarios/storage_u64_bad.scen.json");
}

#[test]
fn storage_usize_rs() {
    world().run("scenarios/storage_usize.scen.json");
}

#[test]
fn storage_usize_bad_rs() {
    world().run("scenarios/storage_usize_bad.scen.json");
}

#[test]
fn struct_eq_rs() {
    world().run("scenarios/struct_eq.scen.json");
}
