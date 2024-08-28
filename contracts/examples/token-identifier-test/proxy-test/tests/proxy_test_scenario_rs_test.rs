use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/proxy-test.mxsc.json", proxy_test::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/proxy_test.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
