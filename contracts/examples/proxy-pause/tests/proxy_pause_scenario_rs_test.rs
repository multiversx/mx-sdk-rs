use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "mxsc:output/proxy-pause.mxsc.json",
        proxy_pause::ContractBuilder,
    );

    blockchain.register_contract(
        "mxsc:../check-pause/output/check-pause.mxsc.json",
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
