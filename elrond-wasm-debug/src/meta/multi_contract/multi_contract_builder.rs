use elrond_wasm::abi::{ContractAbi, EndpointAbi};
use std::collections::{BTreeSet, HashMap, BTreeMap};

use crate::meta::meta_config::MetaConfig;

use super::{ContractMetadataSerde, MultiContractConfigSerde, MultiContractTargetLabelSerde, MultiContractConfig, ContractMetadata, DEFAULT_LABEL};

#[derive(Default)]
struct ContractMetadataBuilder {
    pub config_name: String,
    pub config_wasm_name: String,
    pub labels: BTreeSet<String>,
    pub endpoints: BTreeMap<String, EndpointAbi>,
    pub external_view: bool,
}

impl ContractMetadataBuilder {
    fn map_from_config(
        kvp: (&String, &ContractMetadataSerde),
    ) -> (String, ContractMetadataBuilder) {
        let (config_name, cms) = kvp;
        (
            config_name.clone(),
            ContractMetadataBuilder {
                config_name: config_name.clone(),
                config_wasm_name: cms.wasm_name.clone().unwrap_or_default(),
                external_view: cms.external_view.unwrap_or_default(),
                ..Default::default()
            },
        )
    }

    fn matches_endpoint_labels(&self, endpoint_labels: &[&str]) -> bool {
        if endpoint_labels.is_empty() {
            self.labels.contains(DEFAULT_LABEL)
        } else {
            self.labels.iter().any(|endpoint_label| self.labels.contains(endpoint_label))
        }
    }

    fn wasm_name(&self) -> &String {
        if !self.config_wasm_name.is_empty() {
            &self.config_wasm_name
        } else {
            &self.config_name
        }
    }
}

fn collect_contract_labels(
    contract_builders: &mut HashMap<String, ContractMetadataBuilder>,
    label_targets: &HashMap<String, MultiContractTargetLabelSerde>,
) {
    for (label, targets) in label_targets {
        for target in &targets.0 {
            contract_builders
                .entry(target.clone())
                .or_insert_with(|| ContractMetadataBuilder {
                    config_name: target.clone(),
                    external_view: false, // if unspecified, it should be considered false
                    ..Default::default()
                })
                .labels
                .insert(label.clone());
        }
    }
}

fn collect_endpoint_names(
    contract_builders: &mut HashMap<String, ContractMetadataBuilder>,
    original_abi: &ContractAbi,
) {
    for builder in contract_builders.values_mut() {
        for endpoint_abi in &original_abi.endpoints {
            if builder.matches_endpoint_labels(&endpoint_abi.labels) {
                builder.endpoints.insert(endpoint_abi.name.to_string(), endpoint_abi.clone());
            }
        }
    }
}

fn build_contract_abi(builder: ContractMetadataBuilder, original_abi: &ContractAbi) -> ContractAbi {
    ContractAbi {
        build_info: original_abi.build_info.clone(),
        docs: original_abi.docs,
        name: original_abi.name,
        constructors: Vec::new(),
        endpoints: builder.endpoints.into_values().collect(),
        events: original_abi.events.clone(),
        has_callback: !builder.external_view && original_abi.has_callback,
        type_descriptions: original_abi.type_descriptions.clone(),
    }
}

fn build_contract(builder: ContractMetadataBuilder, crate_config: &MetaConfig) -> ContractMetadata {
    let wasm_name = builder.wasm_name();
    let wasm_crate_name = crate_config.build_args.wasm_name(&crate_config.main_contract.as_ref().unwrap());
    ContractMetadata {
        external_view: builder.external_view,
        wasm_crate_name: format!("wasm-{}", wasm_name),
        wasm_crate_path: format!("./wasm-{}", wasm_name),
        output_name: wasm_name.clone(),
        abi: build_contract_abi(builder, &crate_config.main_contract.as_ref().unwrap().abi),
    }
}

pub fn load_multi_contract_config(
    config: &MultiContractConfigSerde,
    crate_config: &MetaConfig,
) -> MultiContractConfig {
    let mut contract_builders: HashMap<String, ContractMetadataBuilder> =
        config.contracts.iter().map(ContractMetadataBuilder::map_from_config).collect();
    collect_contract_labels(&mut contract_builders, &config.labels);
    let contracts = HashMap::<String, ContractMetadata>::new();
    MultiContractConfig {
        main_contract_name: config.settings.default.clone(),
        contracts: contract_builders.into_values().map(|builder| build_contract(builder, crate_config)).collect(),
    }
}
