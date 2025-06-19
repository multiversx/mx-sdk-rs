use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/mappers/queue-repeat");

    blockchain.register_contract(
        "mxsc:output/queue-repeat.mxsc.json",
        queue_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn queue_repeat_rs() {
    world().run("scenarios/queue_repeat.scen.json");
}

#[test]
fn queue_repeat_struct_rs() {
    world().run("scenarios/queue_repeat_struct.scen.json");
}
