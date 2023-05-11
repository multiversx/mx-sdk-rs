use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:output/bonding-curve-contract.wasm",
        bonding_curve_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_rs() {
    world().run("scenarios/buy.scen.json");
}

#[test]
fn claim_rs() {
    world().run("scenarios/claim.scen.json");
}

#[test]
fn deploy_rs() {
    world().run("scenarios/deploy.scen.json");
}

#[test]
fn deposit_rs() {
    world().run("scenarios/deposit.scen.json");
}

#[test]
fn deposit_more_view_rs() {
    world().run("scenarios/deposit_more_view.scen.json");
}

#[test]
fn sell_rs() {
    world().run("scenarios/sell.scen.json");
}

#[test]
fn set_bonding_curve_rs() {
    world().run("scenarios/set_bonding_curve.scen.json");
}
