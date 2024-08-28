use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/returns-token-identifier-test.mxsc.json", returns_token_identifier_test::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/returns_token_identifier_test.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
