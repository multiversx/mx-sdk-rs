use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/map-repeat.mxsc.json",
        map_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn map_repeat_rs() {
    world().run("scenarios/map_repeat.scen.json");
}

#[test]
fn map_repeat_struct_rs() {
    world().run("scenarios/map_repeat_struct.scen.json");
}
