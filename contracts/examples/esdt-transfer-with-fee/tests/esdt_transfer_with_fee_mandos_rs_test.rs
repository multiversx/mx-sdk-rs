use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/esdt-transfer-with-fee");

    blockchain.register_contract(
        "file:output/esdt-transfer-with-fee.wasm",
        esdt_transfer_with_fee::ContractBuilder,
    );
    blockchain
}

#[test]
fn deploy_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deploy.scen.json", world());
}

#[test]
fn setup_fees_rs() {
    multiversx_sc_scenario::run_rs("scenarios/setup_fees_and_transfer.scen.json", world());
}

#[test]
fn claim_rs() {
    multiversx_sc_scenario::run_rs("scenarios/claim.scen.json", world());
}
