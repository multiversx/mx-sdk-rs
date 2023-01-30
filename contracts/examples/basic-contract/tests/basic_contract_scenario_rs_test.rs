use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "file:output/basic-contract.wasm",
        basic_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn basic_contract_rs() {
    multiversx_sc_scenario::run_rs("scenarios/basic.scen.json", world());
}
