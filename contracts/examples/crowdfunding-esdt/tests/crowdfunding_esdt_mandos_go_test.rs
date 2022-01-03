#[test]
fn crowdfunding_claim_failed_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    elrond_wasm_debug::mandos_go("mandos/crowdfunding-init.scen.json");
}

#[test]
fn egld_crowdfunding_claim_failed_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-claim-failed.scen.json");
}

#[test]
fn egld_crowdfunding_claim_successful_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-claim-successful.scen.json");
}

#[test]
fn egld_crowdfunding_claim_too_early_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn egld_crowdfunding_fund_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-fund.scen.json");
}

#[test]
fn egld_crowdfunding_fund_too_late_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn egld_crowdfunding_init_go() {
    elrond_wasm_debug::mandos_go("mandos/egld-crowdfunding-init.scen.json");
}
