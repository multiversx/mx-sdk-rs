use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn boxed_bytes_zeros_go() {
    world().run("scenarios/boxed_bytes_zeros.scen.json");
}

#[test]
fn echo_async_result_empty_go() {
    world().run("scenarios/echo_async_result_empty.scen.json");
}

#[test]
fn echo_big_int_nested_alloc_go() {
    world().run("scenarios/echo_big_int_nested_alloc.scen.json");
}

#[test]
fn echo_boxed_bytes_go() {
    world().run("scenarios/echo_boxed_bytes.scen.json");
}

#[test]
fn echo_multi_value_tuples_alloc_go() {
    world().run("scenarios/echo_multi_value_tuples_alloc.scen.json");
}

#[test]
fn echo_ser_ex_1_go() {
    world().run("scenarios/echo_ser_ex_1.scen.json");
}

#[test]
fn echo_slice_u_8_go() {
    world().run("scenarios/echo_slice_u8.scen.json");
}

#[test]
fn echo_str_go() {
    world().run("scenarios/echo_str.scen.json");
}

#[test]
fn echo_str_box_go() {
    world().run("scenarios/echo_str_box.scen.json");
}

#[test]
fn echo_string_go() {
    world().run("scenarios/echo_string.scen.json");
}

#[test]
fn echo_varargs_u_32_alloc_go() {
    world().run("scenarios/echo_varargs_u32_alloc.scen.json");
}

#[test]
fn echo_vec_u_8_go() {
    world().run("scenarios/echo_vec_u8.scen.json");
}

#[test]
fn managed_buffer_concat_2_go() {
    world().run("scenarios/managed_buffer_concat_2.scen.json");
}

#[test]
fn managed_buffer_load_slice_go() {
    world().run("scenarios/managed_buffer_load_slice.scen.json");
}

#[test]
fn managed_buffer_overwrite_go() {
    world().run("scenarios/managed_buffer_overwrite.scen.json");
}

#[test]
fn managed_buffer_set_slice_go() {
    world().run("scenarios/managed_buffer_set_slice.scen.json");
}

#[test]
fn only_owner_legacy_go() {
    world().run("scenarios/only_owner_legacy.scen.json");
}

#[test]
fn sc_result_go() {
    world().run("scenarios/sc_result.scen.json");
}

#[test]
fn storage_address_go() {
    world().run("scenarios/storage_address.scen.json");
}

#[test]
fn storage_opt_address_go() {
    world().run("scenarios/storage_opt_address.scen.json");
}

#[test]
fn storage_vec_u_8_go() {
    world().run("scenarios/storage_vec_u8.scen.json");
}
