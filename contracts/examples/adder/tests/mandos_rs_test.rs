use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/adder.wasm",
        Box::new(|context| Box::new(adder::contract_obj(context))),
    );
    contract_map
}

#[test]
fn adder_rs() {
    elrond_wasm_debug::mandos_rs("mandos/adder.scen.json", &contract_map());
}
