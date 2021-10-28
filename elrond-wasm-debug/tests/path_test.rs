use elrond_wasm_debug::*;

fn contract_map_local() -> BlockchainMock {
    BlockchainMock::new()
}

fn contract_map_relative() -> BlockchainMock {
    let mut blockchain = contract_map_local();
    blockchain.set_current_dir_from_workspace("elrond-wasm-debug/tests/mandos/external_steps");

    blockchain
}

/// Checks that externalSteps work fine.
#[test]
fn relative_path_test() {
    elrond_wasm_debug::mandos_rs("external_steps.scen.json", contract_map_relative());
}
