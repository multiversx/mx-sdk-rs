use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/token-release");

    blockchain.register_contract(
        "file:output/token-release.wasm",
        token_release::ContractBuilder,
    );
    blockchain
}

#[test]
fn test_add_group_rs() {
    world().run("scenarios/test-add-group.scen.json");
}

#[test]
fn test_add_user_rs() {
    world().run("scenarios/test-add-user.scen.json");
}

#[test]
fn test_change_user_rs() {
    world().run("scenarios/test-change-user.scen.json");
}

#[test]
fn test_claim_rs() {
    world().run("scenarios/test-claim.scen.json");
}

#[test]
fn test_end_setup_rs() {
    world().run("scenarios/test-end-setup.scen.json");
}

#[test]
fn test_init_rs() {
    world().run("scenarios/test-init.scen.json");
}
