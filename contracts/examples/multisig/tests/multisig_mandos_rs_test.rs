use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
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

#[ignore]
#[test]
fn call_other_shard_1_rs() {
    mx_sc_debug::scenario_rs("scenarios/call_other_shard-1.scen.json", world());
}

#[ignore]
#[test]
fn call_other_shard_2_rs() {
    mx_sc_debug::scenario_rs("scenarios/call_other_shard-2.scen.json", world());
}

#[test]
fn changeboard_rs() {
    mx_sc_debug::scenario_rs("scenarios/changeBoard.scen.json", world());
}

#[test]
fn changequorum_rs() {
    mx_sc_debug::scenario_rs("scenarios/changeQuorum.scen.json", world());
}

#[test]
fn changequorum_toobig_rs() {
    mx_sc_debug::scenario_rs("scenarios/changeQuorum_tooBig.scen.json", world());
}

#[test]
fn deployadder_err_rs() {
    mx_sc_debug::scenario_rs("scenarios/deployAdder_err.scen.json", world());
}

#[test]
fn deployadder_then_call_rs() {
    mx_sc_debug::scenario_rs("scenarios/deployAdder_then_call.scen.json", world());
}

#[test]
fn deployfactorial_rs() {
    mx_sc_debug::scenario_rs("scenarios/deployFactorial.scen.json", world());
}

#[test]
fn deployothermultisig_rs() {
    mx_sc_debug::scenario_rs("scenarios/deployOtherMultisig.scen.json", world());
}

#[test]
fn deploy_duplicate_bm_rs() {
    mx_sc_debug::scenario_rs("scenarios/deploy_duplicate_bm.scen.json", world());
}

#[test]
fn remove_everyone_rs() {
    mx_sc_debug::scenario_rs("scenarios/remove_everyone.scen.json", world());
}

#[test]
fn sendesdt_rs() {
    mx_sc_debug::scenario_rs("scenarios/sendEsdt.scen.json", world());
}

#[test]
fn upgrade_rs() {
    mx_sc_debug::scenario_rs("scenarios/upgrade.scen.json", world());
}

#[test]
fn upgrade_from_source_rs() {
    mx_sc_debug::scenario_rs("scenarios/upgrade_from_source.scen.json", world());
}
