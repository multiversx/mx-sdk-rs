use elrond_wasm::contract_base::CallableContract;
use elrond_wasm_debug::*;

#[test]
fn test_function_selector() {
    let _ = DebugApi::dummy();
    let use_module = use_module::contract_obj::<DebugApi>();

    assert!(!use_module.call("invalid_endpoint"));

    assert!(use_module.call(b"call_mod_a"));
    assert!(use_module.call(b"call_mod_b"));
    assert!(use_module.call(b"call_mod_c"));
    assert!(use_module.call(b"call_contract_base_full_path_endpoint"));
    assert!(use_module.call(b"call_contract_base_endpoint"));
}
