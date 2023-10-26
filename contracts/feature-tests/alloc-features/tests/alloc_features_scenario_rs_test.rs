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
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn boxed_bytes_zeros_rs() {
    world().run("scenarios/boxed_bytes_zeros.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_async_result_empty_rs() {
    world().run("scenarios/echo_async_result_empty.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_big_int_nested_alloc_rs() {
    world().run("scenarios/echo_big_int_nested_alloc.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_boxed_bytes_rs() {
    world().run("scenarios/echo_boxed_bytes.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_multi_value_tuples_alloc_rs() {
    world().run("scenarios/echo_multi_value_tuples_alloc.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_ser_ex_1_rs() {
    world().run("scenarios/echo_ser_ex_1.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_slice_u_8_rs() {
    world().run("scenarios/echo_slice_u8.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_str_rs() {
    world().run("scenarios/echo_str.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_str_box_rs() {
    world().run("scenarios/echo_str_box.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_string_rs() {
    world().run("scenarios/echo_string.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_varargs_u_32_alloc_rs() {
    world().run("scenarios/echo_varargs_u32_alloc.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn echo_vec_u_8_rs() {
    world().run("scenarios/echo_vec_u8.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn managed_buffer_concat_2_rs() {
    world().run("scenarios/managed_buffer_concat_2.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn managed_buffer_load_slice_rs() {
    world().run("scenarios/managed_buffer_load_slice.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn managed_buffer_overwrite_rs() {
    world().run("scenarios/managed_buffer_overwrite.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn managed_buffer_set_slice_rs() {
    world().run("scenarios/managed_buffer_set_slice.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn only_owner_legacy_rs() {
    world().run("scenarios/only_owner_legacy.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn sc_result_rs() {
    world().run("scenarios/sc_result.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn storage_address_rs() {
    world().run("scenarios/storage_address.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn storage_opt_address_rs() {
    world().run("scenarios/storage_opt_address.scen.json");
}

#[test]
#[cfg_attr(any(feature = "single-tx-api", feature = "no-debug-api", feature = "static-api"), ignore)]
fn storage_vec_u_8_rs() {
    world().run("scenarios/storage_vec_u8.scen.json");
}
