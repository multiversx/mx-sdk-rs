use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/set-repeat.wasm",
        Box::new(|context| Box::new(set_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn set_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/set_repeat.scen.json", contract_map());
}
