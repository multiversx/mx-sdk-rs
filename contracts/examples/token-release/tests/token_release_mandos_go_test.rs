#[test]
fn token_release_add_group_go() {
    elrond_wasm_debug::mandos_go("mandos/test-add-group.scen.json");
}

#[test]
fn token_release_add_user_go() {
    elrond_wasm_debug::mandos_go("mandos/test-add-user.scen.json");
}

#[test]
fn token_release_change_user_go() {
    elrond_wasm_debug::mandos_go("mandos/test-change-user.scen.json");
}

#[test]
fn token_release_claim_go() {
    elrond_wasm_debug::mandos_go("mandos/test-claim.scen.json");
}

#[test]
fn token_release_end_setup_go() {
    elrond_wasm_debug::mandos_go("mandos/test-end-setup.scen.json");
}

#[test]
fn token_release_init_go() {
    elrond_wasm_debug::mandos_go("mandos/test-init.scen.json");
}
