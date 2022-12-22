use mx_sc_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/panic-message-features");

    blockchain.register_partial_contract::<panic_message_features::AbiProvider, _>(
        "file:output/panic-message-features.wasm",
        panic_message_features::ContractBuilder,
        "panic-message-features",
    );

    blockchain
}

#[test]
fn panic_message_rs() {
    mx_sc_debug::mandos_rs("mandos/panic-message.scen.json", world());
}
