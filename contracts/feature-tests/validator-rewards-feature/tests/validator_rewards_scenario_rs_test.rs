use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/validator-rewards-feature");
    blockchain.register_contract(
        "mxsc:output/validator-rewards-feature.mxsc.json",
        validator_rewards_feature::ContractBuilder,
    );
    blockchain
}

#[test]
fn validator_rewards_rs() {
    world().run("scenarios/validator_rewards.scen.json");
}
