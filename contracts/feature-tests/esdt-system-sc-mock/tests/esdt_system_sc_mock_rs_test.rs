use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/esdt-system-sc-mock");
    blockchain.register_contract_builder(
        "file:output/esdt-system-sc-mock.wasm",
        esdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    elrond_wasm_debug::mandos_rs("mandos/esdt_system_sc.scen.json", world());
}
