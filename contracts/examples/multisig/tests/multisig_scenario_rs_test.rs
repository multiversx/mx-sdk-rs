use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig.wasm",
        multisig::ContractBuilder,
        "multisig",
    );
    blockchain.register_partial_contract::<multisig::AbiProvider, _>(
        "file:output/multisig-view.wasm",
        multisig::ContractBuilder,
        "multisig-view",
    );

    blockchain.register_contract("file:test-contracts/adder.wasm", adder::ContractBuilder);

    blockchain.register_contract(
        "file:test-contracts/factorial.wasm",
        factorial::ContractBuilder,
    );

    blockchain
}

#[test]
#[ignore]
fn call_other_shard_1_rs() {
    multiversx_sc_scenario::run_rs("scenarios/call_other_shard-1.scen.json", world());
}

#[test]
#[ignore]
fn call_other_shard_2_rs() {
    multiversx_sc_scenario::run_rs("scenarios/call_other_shard-2.scen.json", world());
}

#[test]
fn change_board_rs() {
    multiversx_sc_scenario::run_rs("scenarios/changeBoard.scen.json", world());
}

#[test]
fn change_quorum_rs() {
    multiversx_sc_scenario::run_rs("scenarios/changeQuorum.scen.json", world());
}

#[test]
fn change_quorum_too_big_rs() {
    multiversx_sc_scenario::run_rs("scenarios/changeQuorum_tooBig.scen.json", world());
}

#[test]
fn deploy_adder_err_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deployAdder_err.scen.json", world());
}

#[test]
fn deploy_adder_then_call_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deployAdder_then_call.scen.json", world());
}

#[test]
fn deploy_factorial_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deployFactorial.scen.json", world());
}

#[test]
fn deploy_other_multisig_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deployOtherMultisig.scen.json", world());
}

#[test]
fn deploy_duplicate_bm_rs() {
    multiversx_sc_scenario::run_rs("scenarios/deploy_duplicate_bm.scen.json", world());
}

#[test]
fn remove_everyone_rs() {
    multiversx_sc_scenario::run_rs("scenarios/remove_everyone.scen.json", world());
}

#[test]
fn send_esdt_rs() {
    multiversx_sc_scenario::run_rs("scenarios/sendEsdt.scen.json", world());
}

#[test]
fn upgrade_rs() {
    multiversx_sc_scenario::run_rs("scenarios/upgrade.scen.json", world());
}

#[test]
fn upgrade_from_source_rs() {
    multiversx_sc_scenario::run_rs("scenarios/upgrade_from_source.scen.json", world());
}
