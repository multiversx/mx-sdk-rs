use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/single-value-repeat");

    blockchain.register_contract(
        "mxsc:output/single-value-repeat.mxsc.json",
        single_value_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn single_value_repeat_rs() {
    world().run("scenarios/single_value_repeat.scen.json");
}

#[test]
fn single_value_repeat_struct_rs() {
    world().run("scenarios/single_value_repeat_struct.scen.json");
}
