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
        externalview = true
        wasmname = "multisig-ev.wasm"
        
        [labels]
        default = ["multisig", "multisig-external-view"]
        ev = ["multisig-external-view"]
    "#).unwrap();

    assert_eq!(multi_contract.settings.default, "multisig");
    assert_eq!(multi_contract.labels.default, ["multisig", "multisig-external-view"]);
    assert_eq!(multi_contract.labels.ev, ["multisig-external-view"]);
    assert_eq!(multi_contract.contracts.get("multisig").unwrap().wasmname.is_none(), true);
    assert_eq!(multi_contract.contracts.get("multisig").unwrap().externalview.is_none(), true);
    assert_eq!(multi_contract.contracts.get("multisig-external-view").unwrap().wasmname.as_ref().unwrap(), "multisig-ev.wasm");
    assert_eq!(multi_contract.contracts.get("multisig-external-view").unwrap().externalview.unwrap(), true);
}
