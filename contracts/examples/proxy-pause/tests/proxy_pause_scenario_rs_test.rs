use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/proxy-pause");

    blockchain.register_contract("file:output/proxy-pause.wasm", proxy_pause::ContractBuilder);

    blockchain.register_contract(
        "file:../check-pause/output/check-pause.wasm",
        check_pause::ContractBuilder,
    );
    blockchain
}

#[test]
fn init_rs() {
    world().run("scenarios/init.scen.json");
}

#[test]
fn pause_and_unpause_rs() {
    world().run("scenarios/pause-and-unpause.scen.json");
}
