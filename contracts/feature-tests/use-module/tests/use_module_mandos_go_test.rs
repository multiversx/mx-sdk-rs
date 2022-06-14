#[test]
fn use_module_dns_register_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_features.scen.json");
}

#[test]
fn use_module_internal_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_internal.scen.json");
}

#[test]
fn use_module_only_owner_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_only_owner.scen.json");
}

#[test]
fn use_module_no_endpoint_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_no_endpoint.scen.json");
}

#[test]
fn use_module_pause_go() {
    elrond_wasm_debug::mandos_go("mandos/use_module_pause.scen.json");
}
