use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/big-float-features");

    blockchain.register_contract_builder(
        "file:output/big-float-features.wasm",
        big_float_features::ContractBuilder,
    );

    blockchain
}

// #[test]
// fn big_floats_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/big_floats.scen.json", world());
// }
