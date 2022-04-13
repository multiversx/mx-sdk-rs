use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/formatted-message-features");

    blockchain.register_contract_builder(
        "file:output/formatted-message-features.wasm",
        formatted_message_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn msg_rs() {
    elrond_wasm_debug::mandos_rs("mandos/managed_error_message.scen.json", world());
}

#[test]
fn sc_format_rs() {
    elrond_wasm_debug::mandos_rs("mandos/sc_format.scen.json", world());
}
