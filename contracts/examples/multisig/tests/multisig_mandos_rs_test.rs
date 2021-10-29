use elrond_wasm_debug::*;

fn contract_map() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/multisig");

    blockchain.register_contract(
        "file:output/multisig.wasm",
        Box::new(|context| Box::new(multisig::contract_obj(context))),
    );

    blockchain.register_contract(
        "file:test-contracts/adder.wasm",
        Box::new(|context| Box::new(adder::contract_obj(context))),
    );

    blockchain.register_contract(
        "file:test-contracts/factorial.wasm",
        Box::new(|context| Box::new(factorial::contract_obj(context))),
    );

    blockchain
}

#[test]
fn changeboard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeBoard.scen.json", contract_map());
}

#[test]
fn changequorum_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum.scen.json", contract_map());
}

#[test]
fn changequorum_toobig_rs() {
    elrond_wasm_debug::mandos_rs("mandos/changeQuorum_tooBig.scen.json", contract_map());
}

#[test]
fn deployadder_err_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployAdder_err.scen.json", contract_map());
}
#[test]
fn deploy_from_source_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployOtherMultisig.scen.json", contract_map());
}

#[test]
fn upgrade_rs() {
    elrond_wasm_debug::mandos_rs("mandos/upgrade.scen.json", contract_map());
}

#[test]
fn upgrade_from_source_rs() {
    elrond_wasm_debug::mandos_rs("mandos/upgrade_from_source.scen.json", contract_map());
}

#[test]
fn deployadder_then_call_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployAdder_then_call.scen.json", contract_map());
}

#[test]
fn deployfactorial_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deployFactorial.scen.json", contract_map());
}

#[test]
fn deploy_duplicate_bm_rs() {
    elrond_wasm_debug::mandos_rs("mandos/deploy_duplicate_bm.scen.json", contract_map());
}

#[test]
fn remove_everyone_rs() {
    elrond_wasm_debug::mandos_rs("mandos/remove_everyone.scen.json", contract_map());
}
