#[test]
fn big_float_new_from_frac_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_frac.scen.json");
}

#[test]
fn big_float_new_from_big_int_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_big_int.scen.json");
}

#[test]
fn big_float_new_from_big_uint_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_big_uint.scen.json");
}

#[test]
fn big_float_new_from_int_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_int.scen.json");
}

#[test]
fn big_float_new_from_managed_buffer_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_managed_buffer.scen.json");
}

#[test]
fn big_float_new_from_parts_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_parts.scen.json");
}

#[test]
fn big_float_new_from_sci_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_new_from_sci.scen.json");
}

#[test]
fn big_float_operators_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_operators.scen.json");
}

#[test]
fn big_float_operator_checks_go() {
    elrond_wasm_debug::mandos_go("mandos/big_float_operator_checks.scen.json");
}
