use elrond_wasm_debug::DebugApi;
use elrond_wasm_debug::meta::multi_contract::MultiContract;

#[test]
fn test_serialize_multi_contract() {
    let _ = DebugApi::dummy();

    let multi_contract: MultiContract = toml::from_str(r#"
        [settings]
        default = "multisig"
        
        [contracts]
        multisig = {}
        
        [contracts.multisig-external-view]
        external_view = true
        wasm_name = "multisig-ev.wasm"
        
        [labels]
        default = ["multisig", "multisig-external-view"]
        ev = ["multisig-external-view"]
    "#).unwrap();

    assert_eq!(multi_contract.settings.default, "multisig");
    assert_eq!(multi_contract.labels.default, ["multisig", "multisig-external-view"]);
    assert_eq!(multi_contract.labels.ev, ["multisig-external-view"]);
    assert_eq!(multi_contract.contracts.get("multisig").unwrap().wasm_name.is_none(), true);
    assert_eq!(multi_contract.contracts.get("multisig").unwrap().external_view.is_none(), true);
    assert_eq!(multi_contract.contracts.get("multisig-external-view").unwrap().wasm_name.as_ref().unwrap(), "multisig-ev.wasm");
    assert_eq!(multi_contract.contracts.get("multisig-external-view").unwrap().external_view.unwrap(), true);
}
