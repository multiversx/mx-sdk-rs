#[test]
fn approve_siring_go() {
    elrond_wasm_debug::mandos_go("mandos/approve_siring.scen.json");
}

#[test]
fn breed_ok_go() {
    elrond_wasm_debug::mandos_go("mandos/breed_ok.scen.json");
}

#[test]
fn give_birth_go() {
    elrond_wasm_debug::mandos_go("mandos/give_birth.scen.json");
}

#[test]
fn init_go() {
    elrond_wasm_debug::mandos_go("mandos/init.scen.json");
}

#[test]
fn query_go() {
    elrond_wasm_debug::mandos_go("mandos/query.scen.json");
}

#[test]
fn setup_accounts_go() {
    elrond_wasm_debug::mandos_go("mandos/setup_accounts.scen.json");
}
