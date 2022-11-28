#[test]
fn deploy_erc20_and_crowdfunding_go() {
    elrond_wasm_debug::mandos_go("mandos/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_go() {
    elrond_wasm_debug::mandos_go("mandos/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_go() {
    elrond_wasm_debug::mandos_go("mandos/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_go() {
    elrond_wasm_debug::mandos_go("mandos/fund_without_allowance.scen.json");
}
