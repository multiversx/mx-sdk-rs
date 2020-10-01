
extern crate basic_features;
use basic_features::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/features.wasm",
        Box::new(|mock_ref| Box::new(BasicFeaturesImpl::new(mock_ref))));
    contract_map
}

#[test]
fn return_error() {
    let contract_map = contract_map();
    let mut state = BlockchainMock::new();
    parse_execute_mandos("mandos/return_error.scen.json", &mut state, &contract_map);    
}
