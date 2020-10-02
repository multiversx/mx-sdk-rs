
extern crate use_module;
use use_module::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/use_module.wasm",
        Box::new(|mock_ref| Box::new(UseModuleImpl::new(mock_ref))));
    contract_map
}

#[test]
fn use_module_features() {
    parse_execute_mandos("mandos/use_module_features.scen.json", &contract_map());
}
