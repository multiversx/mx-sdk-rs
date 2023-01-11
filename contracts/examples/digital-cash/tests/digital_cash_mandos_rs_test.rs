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

#[ignore] // verify_ed25519 not implemented
#[test]
fn claim_egld_rs() {
    multiversx_sc_scenario::run_rs("scenarios/claim-egld.scen.json", world());
}

#[ignore] // verify_ed25519 not implemented
#[test]
fn claim_esdt_rs() {
    multiversx_sc_scenario::run_rs("scenarios/claim-esdt.scen.json", world());
}

#[test]
fn fund_egld_and_esdt_rs() {
    multiversx_sc_scenario::run_rs("scenarios/fund-egld-and-esdt.scen.json", world());
}

#[test]
fn set_accounts_rs() {
    multiversx_sc_scenario::run_rs("scenarios/set-accounts.scen.json", world());
}

#[test]
fn withdraw_egld_rs() {
    multiversx_sc_scenario::run_rs("scenarios/withdraw-egld.scen.json", world());
}

#[test]
fn withdraw_esdt_rs() {
    multiversx_sc_scenario::run_rs("scenarios/withdraw-esdt.scen.json", world());
}
