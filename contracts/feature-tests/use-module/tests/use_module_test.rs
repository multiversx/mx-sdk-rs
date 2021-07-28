use elrond_wasm::api::CallableContract;
use elrond_wasm_debug::*;

#[test]
fn test_function_selector() {
    let use_module = use_module::contract_obj(TxContext::dummy());

    assert!(!use_module.call(b"invalid_endpoint"));

    assert!(use_module.call(b"call_mod_a"));
    assert!(use_module.call(b"call_mod_b"));
    assert!(use_module.call(b"call_mod_c"));
}
