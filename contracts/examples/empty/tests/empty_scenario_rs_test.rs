use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/empty.mxsc.json", empty::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty.scen.json");
}
