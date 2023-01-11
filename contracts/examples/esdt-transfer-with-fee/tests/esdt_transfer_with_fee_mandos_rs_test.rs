use mx_sc_scenario::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/esdt-transfer-with-fee");

    blockchain.register_contract(
        "file:output/esdt-transfer-with-fee.wasm",
        esdt_transfer_with_fee::ContractBuilder,
    );
    blockchain
}

#[test]
fn deploy_rs() {
    mx_sc_scenario::scenario_rs("scenarios/deploy.scen.json", world());
}

#[test]
fn setup_fees_rs() {
    mx_sc_scenario::scenario_rs("scenarios/setup_fees_and_transfer.scen.json", world());
}

#[test]
fn claim_rs() {
    mx_sc_scenario::scenario_rs("scenarios/claim.scen.json", world());
}
