// TODO: uncomment after upgrading to VM 1.4.35
// #[test]
// fn claim_egld_go() {
//     elrond_wasm_debug::mandos_go("mandos/claim-egld.scen.json");
// }

// TODO: uncomment after upgrading to VM 1.4.35
// #[test]
// fn claim_esdt_go() {
//     elrond_wasm_debug::mandos_go("mandos/claim-esdt.scen.json");
// }

#[test]
fn fund_egld_and_esdt_go() {
    elrond_wasm_debug::mandos_go("mandos/fund-egld-and-esdt.scen.json");
}

#[test]
fn set_accounts_go() {
    elrond_wasm_debug::mandos_go("mandos/set-accounts.scen.json");
}

#[test]
fn withdraw_egld_go() {
    elrond_wasm_debug::mandos_go("mandos/withdraw-egld.scen.json");
}

#[test]
fn withdraw_esdt_go() {
    elrond_wasm_debug::mandos_go("mandos/withdraw-esdt.scen.json");
}
