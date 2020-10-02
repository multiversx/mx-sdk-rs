
extern crate basic_features;
use basic_features::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/features.wasm",
        Box::new(|mock_ref| Box::new(BasicFeaturesImpl::new(mock_ref))));
    contract_map
}

// #[test]
// fn echo_i32() {
//     parse_execute_mandos("mandos/echo_i32.scen.json", &contract_map());    
// }

#[test]
fn echo_i64() {
    parse_execute_mandos("mandos/echo_i64.scen.json", &contract_map());    
}

#[test]
fn return_error() {
    parse_execute_mandos("mandos/return_error.scen.json", &contract_map());    
}
