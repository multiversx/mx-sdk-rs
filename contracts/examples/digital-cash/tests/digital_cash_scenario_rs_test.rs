use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_egld_rs() {
    world().run("scenarios/claim-egld.scen.json");
}

#[test]
fn claim_esdt_rs() {
    world().run("scenarios/claim-esdt.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn claim_fees_rs() {
    multiversx_sc_scenario::run_rs("scenarios/claim-fees.scen.json", world());
}

#[test]
fn fund_egld_and_esdt_rs() {
    world().run("scenarios/fund-egld-and-esdt.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn withdraw_egld_rs() {
    world().run("scenarios/withdraw-egld.scen.json");
}

#[test]
fn withdraw_esdt_rs() {
    world().run("scenarios/withdraw-esdt.scen.json");
}
