use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/examples/esdt-transfer-with-fee");
    blockchain.register_contract(
        "mxsc:output/esdt-transfer-with-fee.mxsc.json",
        esdt_transfer_with_fee::ContractBuilder,
    );
    blockchain
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
fn setup_fees_and_transfer_rs() {
    world().run("scenarios/setup_fees_and_transfer.scen.json");
}
