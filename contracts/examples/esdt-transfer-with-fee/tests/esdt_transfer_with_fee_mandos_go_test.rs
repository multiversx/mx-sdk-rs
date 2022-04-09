#[test]
fn deploy_go() {
    elrond_wasm_debug::mandos_go("mandos/deploy.scen.json");
}

#[test]
fn setup_fees_go() {
    elrond_wasm_debug::mandos_go("mandos/setup_fees_and_transfer.scen.json");
}
