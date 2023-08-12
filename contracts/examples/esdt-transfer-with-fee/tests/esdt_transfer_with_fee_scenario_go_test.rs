use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn claim_go() {
    world().run("scenarios/claim.scen.json");
}

#[test]
fn deploy_go() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn setup_fees_and_transfer_go() {
    world().run("scenarios/setup_fees_and_transfer.scen.json");
}
