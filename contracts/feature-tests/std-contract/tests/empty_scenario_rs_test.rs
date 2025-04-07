use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/std-contract");
    blockchain.register_contract(
        "mxsc:output/std-contract.mxsc.json",
        std_contract::ContractBuilder,
    );
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/empty.scen.json");
}
