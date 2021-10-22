use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/queue-repeat.wasm",
        Box::new(|context| Box::new(queue_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn queue_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/queue_repeat.scen.json", contract_map());
}
