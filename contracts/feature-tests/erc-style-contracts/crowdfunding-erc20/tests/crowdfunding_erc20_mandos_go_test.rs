#[test]
fn deploy_erc20_and_crowdfunding_go() {
    multiversx_sc_scenario::run_go("scenarios/deploy_erc20_and_crowdfunding.scen.json");
}

#[test]
fn fund_with_insufficient_allowance_go() {
    multiversx_sc_scenario::run_go("scenarios/fund_with_insufficient_allowance.scen.json");
}

#[test]
fn fund_with_sufficient_allowance_go() {
    multiversx_sc_scenario::run_go("scenarios/fund_with_sufficient_allowance.scen.json");
}

#[test]
fn fund_without_allowance_go() {
    multiversx_sc_scenario::run_go("scenarios/fund_without_allowance.scen.json");
}
