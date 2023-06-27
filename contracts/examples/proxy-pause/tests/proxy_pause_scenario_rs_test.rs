use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/proxy-pause");

    blockchain.register_contract("file:output/proxy-pause.wasm", proxy_pause::ContractBuilder);

    blockchain
}

#[test]
fn init_rs() {
    multiversx_sc_scenario::run_rs("scenarios/init.scen.json", world());
}

#[test]
fn pause_and_unpause_rs() {
    multiversx_sc_scenario::run_rs("scenarios/pause-and-unpause.scen.json", world());
}
