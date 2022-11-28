use elrond_wasm_debug::{meta::multi_contract::MultiContract, DebugApi};

#[test]
fn test_serialize_multi_contract() {
    let _ = DebugApi::dummy();

    let multi_contract: MultiContract = toml::from_str(
        r#"
        [settings]
        default = "main_identifier"
        
        [contracts]
        main_identifier = {}
        
        [contracts.c1]
        external_view = true
        wasm_name = "c1-name.wasm"
        
        [labels]
        default = ["main-identifier", "c3", "all"]
        label1 = ["c1", "all"]
        label2 = ["c1", "c2"]
        "*" = ["all"]
    "#,
    )
    .unwrap();

    assert_eq!(multi_contract.settings.default, "main_identifier");

    assert!(multi_contract
        .contracts
        .get("main_identifier")
        .unwrap()
        .wasm_name
        .is_none());

    assert!(multi_contract
        .contracts
        .get("main_identifier")
        .unwrap()
        .external_view
        .is_none());

    assert_eq!(
        multi_contract
            .contracts
            .get("c1")
            .unwrap()
            .wasm_name
            .as_ref()
            .unwrap(),
        "c1-name.wasm"
    );

    assert!(multi_contract
        .contracts
        .get("c1")
        .unwrap()
        .external_view
        .unwrap(),);

    assert_eq!(
        multi_contract.labels.get("default").unwrap().0,
        ["main-identifier", "c3", "all"]
    );
    assert_eq!(
        multi_contract.labels.get("label1").unwrap().0,
        ["c1", "all"]
    );
    assert_eq!(multi_contract.labels.get("label2").unwrap().0, ["c1", "c2"]);
    assert_eq!(multi_contract.labels.get("*").unwrap().0, ["all"]);
}
