use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn buy_go() {
    world().run("scenarios/buy.scen.json");
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
fn deposit_go() {
    world().run("scenarios/deposit.scen.json");
}

#[test]
fn deposit_more_view_go() {
    world().run("scenarios/deposit_more_view.scen.json");
}

#[test]
fn sell_go() {
    world().run("scenarios/sell.scen.json");
}

#[test]
fn set_bonding_curve_go() {
    world().run("scenarios/set_bonding_curve.scen.json");
}
