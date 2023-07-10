use multiversx_sc::contract_base::CallableContract;
use multiversx_sc_scenario::api::StaticApi;

#[test]
fn test_function_selector() {
    let use_module = use_module::contract_obj::<StaticApi>();

    assert!(!use_module.call("invalid_endpoint"));

    assert!(use_module.call("call_mod_a"));
    assert!(use_module.call("call_mod_b"));
    assert!(use_module.call("call_mod_c"));
}
