#[test]
fn changeboard_go() {
    elrond_wasm_debug::mandos_go("mandos/changeBoard.scen.json");
}

#[test]
fn changequorum_go() {
    elrond_wasm_debug::mandos_go("mandos/changeQuorum.scen.json");
}

#[test]
fn changequorum_toobig_go() {
    elrond_wasm_debug::mandos_go("mandos/changeQuorum_tooBig.scen.json");
}

#[test]
fn deployadder_err_go() {
    elrond_wasm_debug::mandos_go("mandos/deployAdder_err.scen.json");
}

#[test]
fn deployadder_then_call_go() {
    elrond_wasm_debug::mandos_go("mandos/deployAdder_then_call.scen.json");
}

#[test]
fn deployfactorial_go() {
    elrond_wasm_debug::mandos_go("mandos/deployFactorial.scen.json");
}

#[test]
fn deployothermultisig_go() {
    elrond_wasm_debug::mandos_go("mandos/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
    elrond_wasm_debug::mandos_go("mandos/deploy_duplicate_bm.scen.json");
}

#[test]
fn remove_everyone_go() {
    elrond_wasm_debug::mandos_go("mandos/remove_everyone.scen.json");
}

#[test]
fn sendesdt_go() {
    elrond_wasm_debug::mandos_go("mandos/sendEsdt.scen.json");
}

#[test]
fn upgrade_go() {
    elrond_wasm_debug::mandos_go("mandos/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_go() {
    elrond_wasm_debug::mandos_go("mandos/upgrade_from_source.scen.json");
}
