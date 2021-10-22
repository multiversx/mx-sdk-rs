use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/map-repeat.wasm",
        Box::new(|context| Box::new(map_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn map_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/map_repeat.scen.json", contract_map());
}
