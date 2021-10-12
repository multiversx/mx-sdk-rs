use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/linked-list-repeat.wasm",
        Box::new(|context| Box::new(linked_list_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn linked_list_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/linked_list_repeat.scen.json", &contract_map());
}
