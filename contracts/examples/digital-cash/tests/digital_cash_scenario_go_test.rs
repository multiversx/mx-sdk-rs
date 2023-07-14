use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_egld_go() {
    world().run("scenarios/claim-egld.scen.json");
}

#[test]
fn claim_esdt_go() {
    world().run("scenarios/claim-esdt.scen.json");
}

#[test]
fn claim_fees_go() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_esdt_go() {
    world().run("scenarios/claim-multi-esdt.scen.json");
}

#[test]
fn forward_go() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_egld_and_esdt_go() {
    world().run("scenarios/fund-egld-and-esdt.scen.json");
}

#[test]
fn set_accounts_go() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_egld_go() {
    world().run("scenarios/withdraw-egld.scen.json");
}

#[test]
fn withdraw_esdt_go() {
    world().run("scenarios/withdraw-esdt.scen.json");
}

#[test]
fn withdraw_multi_esdt_go() {
    world().run("scenarios/withdraw-multi-esdt.scen.json");
}
