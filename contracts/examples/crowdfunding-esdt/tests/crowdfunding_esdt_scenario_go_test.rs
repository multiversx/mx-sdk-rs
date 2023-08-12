use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn generated_fund_go() {
    world().run("scenarios/_generated_fund.scen.json");
}

#[test]
fn generated_init_go() {
    world().run("scenarios/_generated_init.scen.json");
}

#[test]
fn generated_query_status_go() {
    world().run("scenarios/_generated_query_status.scen.json");
}

#[test]
fn generated_sc_err_go() {
    world().run("scenarios/_generated_sc_err.scen.json");
}

#[test]
fn crowdfunding_claim_failed_go() {
    world().run("scenarios/crowdfunding-claim-failed.scen.json");
}

#[test]
fn crowdfunding_claim_successful_go() {
    world().run("scenarios/crowdfunding-claim-successful.scen.json");
}

#[test]
fn crowdfunding_claim_too_early_go() {
    world().run("scenarios/crowdfunding-claim-too-early.scen.json");
}

#[test]
fn crowdfunding_fund_go() {
    world().run("scenarios/crowdfunding-fund.scen.json");
}

#[test]
fn crowdfunding_fund_too_late_go() {
    world().run("scenarios/crowdfunding-fund-too-late.scen.json");
}

#[test]
fn crowdfunding_init_go() {
    world().run("scenarios/crowdfunding-init.scen.json");
}

#[test]
fn egld_crowdfunding_claim_failed_go() {
    world().run("scenarios/egld-crowdfunding-claim-failed.scen.json");
}

#[test]
fn egld_crowdfunding_claim_successful_go() {
    world().run("scenarios/egld-crowdfunding-claim-successful.scen.json");
}

#[test]
fn egld_crowdfunding_claim_too_early_go() {
    world().run("scenarios/egld-crowdfunding-claim-too-early.scen.json");
}

#[test]
fn egld_crowdfunding_fund_go() {
    world().run("scenarios/egld-crowdfunding-fund.scen.json");
}

#[test]
fn egld_crowdfunding_fund_too_late_go() {
    world().run("scenarios/egld-crowdfunding-fund-too-late.scen.json");
}

#[test]
fn egld_crowdfunding_init_go() {
    world().run("scenarios/egld-crowdfunding-init.scen.json");
}
