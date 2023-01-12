use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "file:first-contract/output/first-contract.wasm",
        first_contract::ContractBuilder,
    );

    blockchain.register_contract(
        "file:second-contract/output/second-contract.wasm",
        second_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/init.scen.json", world());
}

#[test]
fn simple_transfer_full_rs() {
    multiversx_sc_scenario::run_rs("scenarios/simple_transfer_full.scen.json", world());
}

#[test]
fn simple_transfer_half_rs() {
    multiversx_sc_scenario::run_rs("scenarios/simple_transfer_half.scen.json", world());
}

#[test]
fn simple_transfer_full_wrong_token_rs() {
    multiversx_sc_scenario::run_rs(
        "scenarios/simple_transfer_full_wrong_token.scen.json",
        world(),
    );
}

// TODO: implement ESDTTransfer + async call
#[ignore]
#[test]
fn rejected_transfer_rs() {
    multiversx_sc_scenario::run_rs("scenarios/reject_transfer.scen.json", world());
}
