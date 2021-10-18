use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/egld-esdt-swap.wasm",
        Box::new(|context| Box::new(egld_esdt_swap::contract_obj(context))),
    );
    contract_map
}

#[test]
fn unwrap_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/unwrap_egld.scen.json", contract_map());
}

#[test]
fn wrap_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/wrap_egld.scen.json", contract_map());
}
