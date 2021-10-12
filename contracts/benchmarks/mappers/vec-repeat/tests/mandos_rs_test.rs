use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/vec-repeat.wasm",
        Box::new(|context| Box::new(vec_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn vec_repeat_mandos_rs() {
    elrond_wasm_debug::mandos_rs("mandos/vec_repeat.scen.json", &contract_map());
}
