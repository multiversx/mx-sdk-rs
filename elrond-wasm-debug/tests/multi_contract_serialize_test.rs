use elrond_wasm_debug::{meta::output_contract::MultiContractConfigSerde, DebugApi};

#[test]
fn test_serialize_multi_contract() {
    let _ = DebugApi::dummy();

    let multi_contract: MultiContractConfigSerde = toml::from_str(
        r#"
        [settings]
        main = "main_id"
        
        [contracts]
        main_id = {}
        
        [contracts.c1]
        name = "c1-name"
        external-view = true

        [contracts.c2]
        add-unlabelled = true
        add-labels = ["label1", "label2"]
        
        [labels-for-contracts]
        default = ["main-identifier", "c1", "c3", "all"]
        label1 = ["c1", "all"]
        label2 = ["c1", "c2"]
    "#,
    )
    .unwrap();

    assert_eq!(multi_contract.settings.main, Some("main_id".to_string()));

    assert_eq!(
        multi_contract
            .contracts
            .get("main_id")
            .unwrap()
            .name
            .is_none(),
        true
    );
    assert_eq!(
        multi_contract
            .contracts
            .get("main_id")
            .unwrap()
            .external_view,
        None
    );
    assert_eq!(
        multi_contract.contracts.get("c1").unwrap().name,
        Some("c1-name".to_string())
    );
    assert_eq!(
        multi_contract.contracts.get("c1").unwrap().external_view,
        Some(true)
    );

    assert_eq!(
        multi_contract.contracts.get("c2").unwrap().add_unlabelled,
        Some(true)
    );

    assert_eq!(
        multi_contract.contracts.get("c2").unwrap().add_labels,
        vec!["label1", "label2"]
    );

    assert_eq!(
        multi_contract.labels_for_contracts.get("default").unwrap(),
        &vec!["main-identifier", "c1", "c3", "all"]
    );
    assert_eq!(
        multi_contract.labels_for_contracts.get("label1").unwrap(),
        &vec!["c1", "all"]
    );
    assert_eq!(
        multi_contract.labels_for_contracts.get("label2").unwrap(),
        &vec!["c1", "c2"]
    );
}
