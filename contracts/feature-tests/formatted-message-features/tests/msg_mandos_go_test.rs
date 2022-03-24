#[test]
fn msg_go() {
    elrond_wasm_debug::mandos_go("mandos/managed_error_message.scen.json");
}

#[test]
fn sc_format_go() {
    elrond_wasm_debug::mandos_go("mandos/sc_format.scen.json");
}
