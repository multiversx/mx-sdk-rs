use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("file:output/str-repeat.wasm", str_repeat::ContractBuilder);
    blockchain
}

#[test]
fn test_str_repeat_rs() {
    multiversx_sc_scenario::run_rs("scenarios/str_repeat.scen.json", world());
}
