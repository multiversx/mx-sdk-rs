use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/multi-contract-features");

    blockchain.register_contract_builder(
        "file:output/multi-contract-features.wasm",
        multi_contract_features::ContractBuilder,
    );
    blockchain.register_external_view_contract_builder(
        "file:output/multi-contract-features-view.wasm",
        multi_contract_features::ContractBuilder,
    );
    blockchain
}

#[test]
fn external_pure_rs() {
    elrond_wasm_debug::mandos_rs("mandos/external-pure.scen.json", world());
}
