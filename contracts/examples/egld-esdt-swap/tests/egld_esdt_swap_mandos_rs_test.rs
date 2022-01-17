use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/egld-esdt-swap.wasm",
        egld_esdt_swap::ContractBuilder,
    );
    blockchain
}

#[test]
fn unwrap_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unwrap_egld.scen.json", world());
}

#[test]
fn wrap_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/wrap_egld.scen.json", world());
}
