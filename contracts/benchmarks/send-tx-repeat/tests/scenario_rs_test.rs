use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/benchmarks/send-tx-repeat");
    blockchain.register_contract(
        "mxsc:output/send-tx-repeat.mxsc.json",
        send_tx_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn send_tx_repeat_rs() {
    world().run("scenarios/send_tx_repeat.scen.json");
}
