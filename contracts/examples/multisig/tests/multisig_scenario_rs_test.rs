use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(
        ScenarioExecutorConfig::Debugger.then(ScenarioExecutorConfig::Experimental),
    );
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "mxsc:output/multisig.mxsc.json",
        multisig::ContractBuilder,
        "multisig",
    );
    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "mxsc:output/multisig-view.mxsc.json",
        multisig::ContractBuilder,
        "multisig-view",
    );

    blockchain
}

#[test]
#[ignore]
fn call_other_shard_1_rs() {
    world().run("scenarios/call_other_shard-1.scen.json");
}

#[test]
#[ignore]
fn call_other_shard_2_rs() {
    world().run("scenarios/call_other_shard-2.scen.json");
}

#[test]
fn change_board_rs() {
    world().run("scenarios/changeBoard.scen.json");
}

#[test]
fn change_quorum_rs() {
    world().run("scenarios/changeQuorum.scen.json");
}

#[test]
fn change_quorum_too_big_rs() {
    world().run("scenarios/changeQuorum_tooBig.scen.json");
}

#[test]
fn deploy_adder_err_rs() {
    world().run("scenarios/deployAdder_err.scen.json");
}

#[test]
fn deploy_adder_then_call_rs() {
    world().run("scenarios/deployAdder_then_call.scen.json");
}

#[test]
fn deploy_factorial_rs() {
    world().run("scenarios/deployFactorial.scen.json");
}

#[test]
fn deploy_other_multisig_rs() {
    world().run("scenarios/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_rs() {
    world().run("scenarios/deploy_duplicate_bm.scen.json");
}

#[test]
fn interactor_nft_rs() {
    world().run("scenarios/interactor_nft.scen.json");
}

#[test]
fn interactor_nft_all_roles_rs() {
    world().run("scenarios/interactor_nft_all_roles.scen.json");
}

#[test]
fn interactor_wegld_rs() {
    world().run("scenarios/interactor_wegld.scen.json");
}

#[test]
fn remove_everyone_rs() {
    world().run("scenarios/remove_everyone.scen.json");
}

#[test]
fn send_esdt_rs() {
    world().run("scenarios/sendEsdt.scen.json");
}

#[test]
fn upgrade_rs() {
    world().run("scenarios/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_rs() {
    world().run("scenarios/upgrade_from_source.scen.json");
}
