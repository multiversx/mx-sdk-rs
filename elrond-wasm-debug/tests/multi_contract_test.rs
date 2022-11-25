use elrond_wasm::abi::{ContractAbi, EndpointAbi};
use elrond_wasm_debug::{
    meta::output_contract::{MultiContractConfigSerde, OutputContractConfig},
    DebugApi,
};

fn get_serialized_toml() -> MultiContractConfigSerde {
    toml::from_str(
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
    .unwrap()
}

#[test]
fn test_serialize_multi_contract() {
    let _ = DebugApi::dummy();

    let multi_contract = get_serialized_toml();

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

fn get_contract_abi() -> ContractAbi {
    let endpoints = vec![
        EndpointAbi::generate_with_name_and_labels("endpoint1", &["label1"]),
        EndpointAbi::generate_with_name_and_labels("endpoint1", &["label2"]),
        EndpointAbi::generate_with_name_and_labels("endpoint2", &["label2"]),
        EndpointAbi::generate_with_name_and_labels("endpoint3", &["label2"]),
        EndpointAbi::generate_with_name_and_labels("endpoint4", &["main-id"]),
        EndpointAbi::generate_with_name_and_labels("endpoint5", &["label1"]),
    ];
    ContractAbi::generate_with_endpoints(endpoints)
}

#[test]
fn test_output_contract_config() {
    let serde = get_serialized_toml();
    let abi = get_contract_abi();

    let contract_config = OutputContractConfig::load_from_config(&serde, &abi);

    assert_eq!(contract_config.default_contract_config_name, "main_id");
    assert_eq!(contract_config.contracts.len(), 6);
}
