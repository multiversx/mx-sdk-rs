use elrond_wasm::contract_base::CallableContract;
use elrond_wasm_debug::*;

#[test]
fn test_function_selector() {
    let _ = DebugApi::dummy();
    let use_module = use_module::contract_obj::<DebugApi>();

    assert!(!use_module.call("invalid_endpoint"));

    assert!(use_module.call("call_mod_a"));
    assert!(use_module.call("call_mod_b"));
    assert!(use_module.call("call_mod_c"));
}
