#[test]
fn external_pure_go() {
    elrond_wasm_debug::mandos_go("mandos/external-pure.scen.json");
}

#[test]
fn external_get_go() {
    elrond_wasm_debug::mandos_go("mandos/external-get.scen.json");
}
