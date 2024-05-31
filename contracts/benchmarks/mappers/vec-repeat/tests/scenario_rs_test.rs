use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/vec-repeat.mxsc.json",
        vec_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn vec_repeat_rs() {
    world().run("scenarios/vec_repeat.scen.json");
}

#[test]
fn vec_repeat_struct_rs() {
    world().run("scenarios/vec_repeat_struct.scen.json");
}
