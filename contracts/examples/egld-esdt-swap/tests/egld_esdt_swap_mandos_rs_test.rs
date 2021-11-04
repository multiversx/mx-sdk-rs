use elrond_wasm::*;
use elrond_wasm_debug::*;

#[allow(unused)]
fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract(
        "file:output/egld-esdt-swap.wasm",
        Box::new(|context| Box::new(egld_esdt_swap::contract_obj(context))),
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
