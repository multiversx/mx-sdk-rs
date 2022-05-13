use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/alloc-features");

    blockchain.register_contract_builder(
        "file:output/alloc-features.wasm",
        alloc_features::ContractBuilder,
    );

    blockchain
}
#[test]
fn boxed_bytes_zeros_rs() {
    elrond_wasm_debug::mandos_rs("mandos/boxed_bytes_zeros.scen.json", world());
}

// #[test]
// fn crypto_elliptic_curves_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_elliptic_curves.scen.json", world());
// }

#[test]
fn crypto_keccak256_legacy_alloc_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crypto_keccak256_legacy_alloc.scen.json", world());
}

// #[test]
// fn crypto_ripemd160_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_ripemd160.scen.json", world());
// }

#[test]
fn crypto_sha256_legacy_alloc_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crypto_sha256_legacy_alloc.scen.json", world());
}

// #[test]
// fn crypto_verify_bls_legacy_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_verify_bls_legacy.scen.json", world());
// }

#[test]
fn crypto_verify_ed25519_legacy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/crypto_verify_ed25519_legacy.scen.json", world());
}

// #[test]
// fn crypto_verify_secp256k1_legacy_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/crypto_verify_secp256k1_legacy.scen.json", world());
// }

#[test]
fn echo_async_result_empty_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_async_result_empty.scen.json", world());
}

#[test]
fn echo_big_int_nested_alloc_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_big_int_nested_alloc.scen.json", world());
}

#[test]
fn echo_boxed_bytes_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_boxed_bytes.scen.json", world());
}

#[test]
fn echo_multi_value_tuples_alloc_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_multi_value_tuples_alloc.scen.json", world());
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
fn echo_varargs_u32_alloc_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_varargs_u32_alloc.scen.json", world());
}

#[test]
fn echo_vec_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/echo_vec_u8.scen.json", world());
}

#[test]
fn events_legacy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/events_legacy.scen.json", world());
}

#[test]
fn managed_buffer_concat_2_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_concat_2.scen.json", world());
}

#[test]
fn managed_buffer_load_slice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_load_slice.scen.json", world());
}

#[test]
fn managed_buffer_overwrite_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_overwrite.scen.json", world());
}

#[test]
fn managed_buffer_set_slice_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_buffer_set_slice.scen.json", world());
}

#[test]
fn only_owner_legacy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/only_owner_legacy.scen.json", world());
}

#[test]
fn sc_result_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sc_result.scen.json", world());
}

#[test]
fn storage_address_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_address.scen.json", world());
}

#[test]
fn storage_opt_address_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_opt_address.scen.json", world());
}

#[test]
fn storage_vec_u8_rs() {
    elrond_wasm_debug::mandos_rs("mandos/storage_vec_u8.scen.json", world());
}
