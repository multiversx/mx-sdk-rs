use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/alloc-features");

    blockchain.register_contract(
        "file:output/alloc-features.wasm",
        alloc_features::ContractBuilder,
    );

    blockchain
}
#[test]
fn boxed_bytes_zeros_rs() {
    multiversx_sc_scenario::run_rs("scenarios/boxed_bytes_zeros.scen.json", world());
}

#[ignore]
#[test]
fn crypto_elliptic_curves_legacy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_elliptic_curves_legacy.scen.json", world());
}

#[test]
fn crypto_keccak256_legacy_alloc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_keccak256_legacy_alloc.scen.json", world());
}

#[ignore]
#[test]
fn crypto_ripemd160_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_ripemd160.scen.json", world());
}

#[test]
fn crypto_sha256_legacy_alloc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_sha256_legacy_alloc.scen.json", world());
}

#[ignore]
#[test]
fn crypto_verify_bls_legacy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_verify_bls_legacy.scen.json", world());
}

#[test]
fn crypto_verify_ed25519_legacy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/crypto_verify_ed25519_legacy.scen.json", world());
}

#[ignore]
#[test]
fn crypto_verify_secp256k1_legacy_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/crypto_verify_secp256k1_legacy.scen.json",
        world(),
    );
}

#[test]
fn echo_async_result_empty_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_async_result_empty.scen.json", world());
}

#[test]
fn echo_big_int_nested_alloc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_big_int_nested_alloc.scen.json", world());
}

#[test]
fn echo_boxed_bytes_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_boxed_bytes.scen.json", world());
}

#[test]
fn echo_multi_value_tuples_alloc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_multi_value_tuples_alloc.scen.json", world());
}

#[test]
fn echo_ser_ex_1_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_ser_ex_1.scen.json", world());
}

#[test]
fn echo_slice_u8_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_slice_u8.scen.json", world());
}

#[test]
fn echo_str_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_str.scen.json", world());
}

#[test]
fn echo_str_box_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_str_box.scen.json", world());
}

#[test]
fn echo_string_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_string.scen.json", world());
}

#[test]
fn echo_varargs_u32_alloc_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_varargs_u32_alloc.scen.json", world());
}

#[test]
fn echo_vec_u8_rs() {
    multiversx_sc_scenario::run_rs("scenarios/echo_vec_u8.scen.json", world());
}

#[test]
fn events_legacy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/events_legacy.scen.json", world());
}

#[test]
fn managed_buffer_concat_2_rs() {
    multiversx_sc_scenario::run_rs("scenarios/managed_buffer_concat_2.scen.json", world());
}

#[test]
fn managed_buffer_load_slice_rs() {
    multiversx_sc_scenario::run_rs("scenarios/managed_buffer_load_slice.scen.json", world());
}

#[test]
fn managed_buffer_overwrite_rs() {
    multiversx_sc_scenario::run_rs("scenarios/managed_buffer_overwrite.scen.json", world());
}

#[test]
fn managed_buffer_set_slice_rs() {
    multiversx_sc_scenario::run_rs("scenarios/managed_buffer_set_slice.scen.json", world());
}

#[test]
fn only_owner_legacy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/only_owner_legacy.scen.json", world());
}

#[test]
fn sc_result_rs() {
    multiversx_sc_scenario::run_rs("scenarios/sc_result.scen.json", world());
}

#[test]
fn storage_address_rs() {
    multiversx_sc_scenario::run_rs("scenarios/storage_address.scen.json", world());
}

#[test]
fn storage_opt_address_rs() {
    multiversx_sc_scenario::run_rs("scenarios/storage_opt_address.scen.json", world());
}

#[test]
fn storage_vec_u8_rs() {
    multiversx_sc_scenario::run_rs("scenarios/storage_vec_u8.scen.json", world());
}
