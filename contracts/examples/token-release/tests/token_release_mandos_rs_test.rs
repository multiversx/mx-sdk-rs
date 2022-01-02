use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/token-release");

    blockchain.register_contract_builder(
        "file:output/token-release.wasm",
        token_release::contract_builder,
    );
    blockchain
}

#[test]
fn token_release_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-init.scen.json", world());
}

#[test]
fn token_release_add_group_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-add-group.scen.json", world());
}

#[test]
fn token_release_add_user_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-add-user.scen.json", world());
}

#[test]
fn token_release_end_setup_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-end-setup.scen.json", world());
}

#[test]
fn token_release_claim_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-claim.scen.json", world());
}

#[test]
fn token_release_change_user_rs() {
    elrond_wasm_debug::mandos_rs("mandos/test-change-user.scen.json", world());
}
