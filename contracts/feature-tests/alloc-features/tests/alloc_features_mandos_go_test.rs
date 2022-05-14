#[test]
fn boxed_bytes_zeros_go() {
    elrond_wasm_debug::mandos_go("mandos/boxed_bytes_zeros.scen.json");
}

#[test]
fn crypto_elliptic_curves_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_elliptic_curves_legacy.scen.json");
}

#[test]
fn crypto_keccak256_legacy_alloc_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_keccak256_legacy_alloc.scen.json");
}

#[test]
fn crypto_ripemd160_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_ripemd160_legacy.scen.json");
}

#[test]
fn crypto_sha256_legacy_alloc_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_sha256_legacy_alloc.scen.json");
}

#[test]
fn crypto_verify_bls_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_bls_legacy.scen.json");
}

#[test]
fn crypto_verify_ed25519_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_ed25519_legacy.scen.json");
}

#[test]
fn crypto_verify_secp256k1_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/crypto_verify_secp256k1_legacy.scen.json");
}

#[test]
fn echo_async_result_empty_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_async_result_empty.scen.json");
}

#[test]
fn echo_big_int_nested_alloc_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_big_int_nested_alloc.scen.json");
}

#[test]
fn echo_boxed_bytes_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_boxed_bytes.scen.json");
}

#[test]
fn echo_multi_value_tuples_alloc_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_multi_value_tuples_alloc.scen.json");
}

#[test]
fn echo_ser_ex_1_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_ser_ex_1.scen.json");
}

#[test]
fn echo_slice_u8_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_slice_u8.scen.json");
}

#[test]
fn echo_str_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_str.scen.json");
}

#[test]
fn echo_str_box_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_str_box.scen.json");
}

#[test]
fn echo_string_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_string.scen.json");
}

#[test]
fn echo_varargs_u32_alloc_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_varargs_u32_alloc.scen.json");
}

#[test]
fn echo_vec_u8_go() {
    elrond_wasm_debug::mandos_go("mandos/echo_vec_u8.scen.json");
}

#[test]
fn events_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/events_legacy.scen.json");
}

#[test]
fn managed_buffer_concat_2_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_concat_2.scen.json");
}

#[test]
fn managed_buffer_load_slice_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_load_slice.scen.json");
}

#[test]
fn managed_buffer_overwrite_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_overwrite.scen.json");
}

#[test]
fn managed_buffer_set_slice_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_buffer_set_slice.scen.json");
}

#[test]
fn only_owner_legacy_go() {
    elrond_wasm_debug::mandos_go("mandos/only_owner_legacy.scen.json");
}

#[test]
fn sc_result_go() {
    elrond_wasm_debug::mandos_go("mandos/sc_result.scen.json");
}

#[test]
fn storage_address_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_address.scen.json");
}

#[test]
fn storage_opt_address_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_opt_address.scen.json");
}

#[test]
fn storage_vec_u8_go() {
    elrond_wasm_debug::mandos_go("mandos/storage_vec_u8.scen.json");
}
