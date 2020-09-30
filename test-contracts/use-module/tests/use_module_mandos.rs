
extern crate use_module;
use use_module::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn mock_state() -> ArwenMockRef {
    let mock_ref = ArwenMockState::new_ref();
    mock_ref.register_contract(
        "file:../output/use_module.wasm",
        Box::new(|mock_ref| Box::new(UseModuleImpl::new(mock_ref))));
    mock_ref
}

#[test]
fn use_module_features() {
    parse_execute_mandos(&mock_state(), "mandos/use_module_features.scen.json");

}
