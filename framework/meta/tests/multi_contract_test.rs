use multiversx_sc::abi::{ContractAbi, EndpointAbi};
use multiversx_sc_meta::cmd::contract::output_contract::{
    MultiContractConfigSerde, OutputContractGlobalConfig,
};

fn get_serialized_toml() -> MultiContractConfigSerde {
    toml::from_str(
        r#"
        [settings]
        main = "main-contract"
        
        [contracts.main-contract]
        add-unlabelled = true
        add-labels = ["label1"]

        [contracts.secondary-contract]
        name = "contract2-name"
        add-labels = ["label1", "label2"]
        external-view = true
        
        [labels-for-contracts]
        default = ["main-contract"]
        label1 = ["main-contract", "secondary-contract"]
        label2 = ["secondary-contract"]
    "#,
    )
    .unwrap()
}

fn get_contract_abi() -> ContractAbi {
    let endpoints = vec![
        EndpointAbi::endpoint_with_name_and_labels("endpoint1", &["label1", "label2"]),
        EndpointAbi::endpoint_with_name_and_labels("endpoint2", &["label2"]),
        EndpointAbi::endpoint_with_name_and_labels("endpoint3", &["label2"]),
        EndpointAbi::endpoint_with_name_and_labels("endpoint4", &["label2"]),
        EndpointAbi::endpoint_with_name_and_labels("endpoint5", &[]), // unlabeled endpoint, should end up in main contract
    ];
    ContractAbi::generate_with_endpoints(endpoints)
}

#[test]
fn test_serialize_multi_contract() {
    let multi_contract = get_serialized_toml();

    assert_eq!(
        multi_contract.settings.main,
        Some("main-contract".to_string())
    );

    assert_eq!(
        multi_contract
            .contracts
            .get("main-contract")
            .unwrap()
            .external_view,
        None
    );
    assert_eq!(
        multi_contract
            .contracts
            .get("secondary-contract")
            .unwrap()
            .name,
        Some("contract2-name".to_string())
    );
    assert_eq!(
        multi_contract
            .contracts
            .get("secondary-contract")
            .unwrap()
            .external_view,
        Some(true)
    );

    assert_eq!(
        multi_contract.labels_for_contracts.get("default").unwrap(),
        &vec!["main-contract"]
    );
    assert_eq!(
        multi_contract.labels_for_contracts.get("label1").unwrap(),
        &vec!["main-contract", "secondary-contract"]
    );
    assert_eq!(
        multi_contract.labels_for_contracts.get("label2").unwrap(),
        &vec!["secondary-contract"]
    );
}

#[test]
fn test_output_contract_config() {
    let serde = get_serialized_toml();
    let abi = get_contract_abi();

    let contract_config = OutputContractGlobalConfig::load_from_config(&serde, &abi);

    assert_eq!(
        contract_config.default_contract_config_name,
        "main-contract"
    );
    assert_eq!(contract_config.contracts.len(), 2);
    assert!(contract_config
        .get_contract_by_id("secondary-contract".to_string())
        .is_some());
    assert!(contract_config
        .get_contract_by_id("unexisting-contract]".to_string())
        .is_none());
    assert!(contract_config
        .get_contract_by_name("contract2-name".to_string())
        .is_some());
    assert!(contract_config
        .get_contract_by_name("contract-wrong-name]".to_string())
        .is_none());

    let main_contract = contract_config.main_contract();
    assert_eq!(main_contract.contract_id, "main-contract");
    assert_eq!(main_contract.contract_name, "main-contract");

    assert_eq!(main_contract.endpoint_names(), ["endpoint5", "endpoint1"]);
    assert_eq!(
        contract_config
            .get_contract_by_name("contract2-name".to_string())
            .unwrap()
            .endpoint_names(),
        ["endpoint1", "endpoint2", "endpoint3", "endpoint4"]
    );
}
