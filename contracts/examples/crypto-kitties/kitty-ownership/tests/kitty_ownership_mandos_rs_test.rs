use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();

    blockchain.register_contract_builder(
        "file:../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
        kitty_genetic_alg::contract_builder,
    );
    blockchain.register_contract_builder(
        "file:output/kitty-ownership.wasm",
        kitty_ownership::contract_builder,
    );

    blockchain
}

#[test]
fn approve_siring_rs() {
    elrond_wasm_debug::mandos_rs("mandos/approve_siring.scen.json", world());
}

#[test]
fn breed_ok_rs() {
    elrond_wasm_debug::mandos_rs("mandos/breed_ok.scen.json", world());
}

#[test]
fn give_birth_rs() {
    elrond_wasm_debug::mandos_rs("mandos/give_birth.scen.json", world());
}

#[test]
fn init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/init.scen.json", world());
}

#[test]
fn query_rs() {
    elrond_wasm_debug::mandos_rs("mandos/query.scen.json", world());
}

#[test]
fn setup_accounts_rs() {
    elrond_wasm_debug::mandos_rs("mandos/setup_accounts.scen.json", world());
}
