#[test]
fn unwrap_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/unwrap_egld.scen.json");
}

#[test]
fn wrap_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/wrap_egld.scen.json");
}
