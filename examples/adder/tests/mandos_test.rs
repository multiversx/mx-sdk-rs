
extern crate adder;
use adder::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

#[test]
fn test_mandos() {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/adder.wasm",
        Box::new(|mock_ref| Box::new(AdderImpl::new(mock_ref))));

    let mock_ref = ArwenMockState::new_ref();

    parse_execute_mandos("mandos/adder.scen.json", &mock_ref, &contract_map);

    println!("Ok");

}
