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
fn deploy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deploy.scen.json", world());
}

#[test]
fn deposit_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deposit.scen.json", world());
}

#[test]
fn set_bonding_curve_rs() {
    multiversx_sc_scenario::run_rs("scenarios/set_bonding_curve.scen.json", world());
}

#[test]
fn buy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/buy.scen.json", world());
}

#[test]
fn sell_rs() {
    multiversx_sc_scenario::run_rs("scenarios/sell.scen.json", world());
}

#[test]
fn deposit_more_view_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deposit_more_view.scen.json", world());
}

#[test]
fn claim_rs() {
    multiversx_sc_scenario::run_rs("scenarios/claim.scen.json", world());
}
