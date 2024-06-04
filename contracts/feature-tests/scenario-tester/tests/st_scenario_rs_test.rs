use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/scenario-tester.mxsc.json",
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}

#[test]
fn st_adder_rs() {
    world().run("scenarios/st-adder.scen.json");
}
