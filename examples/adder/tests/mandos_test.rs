
extern crate adder;
use adder::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

#[test]
fn test_mandos() {
    let mock_ref = ArwenMockState::new_ref();

    mock_ref.register_contract(
        "file:../output/adder.wasm",
        Box::new(|mock_ref| Box::new(AdderImpl::new(mock_ref))));

    parse_execute_mandos(&mock_ref, "mandos/adder.scen.json");

    mock_ref.clear_state();
    println!("Ok");

}
