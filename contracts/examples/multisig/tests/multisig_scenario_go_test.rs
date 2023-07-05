use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn call_other_shard_1_go() {
    world().run("scenarios/call_other_shard-1.scen.json");
}

#[test]
fn call_other_shard_2_go() {
    world().run("scenarios/call_other_shard-2.scen.json");
}

#[test]
fn change_board_go() {
    world().run("scenarios/changeBoard.scen.json");
}

#[test]
fn change_quorum_go() {
    world().run("scenarios/changeQuorum.scen.json");
}

#[test]
fn change_quorum_too_big_go() {
    world().run("scenarios/changeQuorum_tooBig.scen.json");
}

#[test]
fn deploy_adder_err_go() {
    world().run("scenarios/deployAdder_err.scen.json");
}

#[test]
fn deploy_adder_then_call_go() {
    world().run("scenarios/deployAdder_then_call.scen.json");
}

#[test]
fn deploy_factorial_go() {
    world().run("scenarios/deployFactorial.scen.json");
}

#[test]
fn deploy_other_multisig_go() {
    world().run("scenarios/deployOtherMultisig.scen.json");
}

#[test]
fn deploy_duplicate_bm_go() {
    world().run("scenarios/deploy_duplicate_bm.scen.json");
}

#[test]
#[ignore = "system SC not yet implemented"]
fn interactor_nft_go() {
    world().run("scenarios/interactor_nft.scen.json");
}

#[test]
#[ignore = "system SC not yet implemented"]
fn interactor_nft_all_roles_go() {
    world().run("scenarios/interactor_nft_all_roles.scen.json");
}

#[test]
fn interactor_wegld_go() {
    world().run("scenarios/interactor_wegld.scen.json");
}

#[test]
fn remove_everyone_go() {
    world().run("scenarios/remove_everyone.scen.json");
}

// TODO: investigate gas issue
#[test]
#[ignore]
fn send_esdt_go() {
    world().run("scenarios/sendEsdt.scen.json");
}

#[test]
fn upgrade_go() {
    world().run("scenarios/upgrade.scen.json");
}

#[test]
fn upgrade_from_source_go() {
    world().run("scenarios/upgrade_from_source.scen.json");
}
