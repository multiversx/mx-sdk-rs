use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/multi-contract-features");

    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features",
    );
    blockchain.register_partial_contract::<multi_contract_features::AbiProvider, _>(
        "file:output/multi-contract-features-view.wasm",
        multi_contract_features::ContractBuilder,
        "multi-contract-features-view",
    );

    blockchain
}

#[test]
fn external_pure_rs() {
    mx_sc_debug::mandos_rs("mandos/external-pure.scen.json", world());
}

#[test]
fn external_get_rs() {
    mx_sc_debug::mandos_rs("mandos/external-get.scen.json", world());
}
