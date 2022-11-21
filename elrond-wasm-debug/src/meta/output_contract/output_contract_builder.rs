use elrond_wasm::abi::{ContractAbi, EndpointAbi};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fs,
    path::Path,
};

use super::{
    ContractMetadataSerde, MultiContractConfigSerde, MultiContractTargetLabelSerde, OutputContract,
    OutputContractConfig, DEFAULT_LABEL,
};

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
            self.labels
                .iter()
                .any(|contract_label| endpoint_labels.contains(&contract_label.as_str()))
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

fn collect_endpoints(
    contract_builders: &mut HashMap<String, ContractMetadataBuilder>,
    original_abi: &ContractAbi,
) {
    for builder in contract_builders.values_mut() {
        for endpoint_abi in &original_abi.endpoints {
            if builder.matches_endpoint_labels(&endpoint_abi.labels) {
                builder
                    .endpoints
                    .insert(endpoint_abi.name.to_string(), endpoint_abi.clone());
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

fn build_contract(builder: ContractMetadataBuilder, original_abi: &ContractAbi) -> OutputContract {
    let name = builder.wasm_name().clone();
    OutputContract {
        main: false,
        external_view: builder.external_view,
        config_name: builder.config_name.clone(),
        public_name: name,
        abi: build_contract_abi(builder, original_abi),
        cargo_toml_contents_cache: None,
    }
}

fn set_main_contract_flag(
    contracts: &mut Vec<OutputContract>,
    default_contract_config_name_opt: &Option<String>,
) {
    if let Some(default_contract_config_name) = default_contract_config_name_opt {
        for contract in contracts.iter_mut() {
            if &contract.config_name == default_contract_config_name {
                contract.main = true;
                return;
            }
        }

        panic!(
            "Could not find default contract '{}' among the output contracts. Available contracts are: {:?}",
            default_contract_config_name,
            contracts.iter().map(|contract| &contract.config_name).collect::<Vec<_>>(),
        )
    } else {
        let first_contract = contracts.get_mut(0).unwrap_or_else(|| {
            panic!("Cannot set default contract because no optput contract was specified.")
        });
        first_contract.main = true;
    }
}

impl OutputContractConfig {
    pub fn load_from_config(config: &MultiContractConfigSerde, original_abi: &ContractAbi) -> Self {
        let mut contract_builders: HashMap<String, ContractMetadataBuilder> = config
            .contracts
            .iter()
            .map(ContractMetadataBuilder::map_from_config)
            .collect();
        collect_contract_labels(&mut contract_builders, &config.labels);
        collect_endpoints(&mut contract_builders, original_abi);
        let mut contracts: Vec<OutputContract> = contract_builders
            .into_values()
            .map(|builder| build_contract(builder, original_abi))
            .collect();
        set_main_contract_flag(&mut contracts, &config.settings.main);
        OutputContractConfig {
            default_contract_config_name: config.settings.main.clone().unwrap_or_default(),
            contracts,
        }
    }

    pub fn default_config(original_abi: &ContractAbi) -> Self {
        let default_contract_config_name = original_abi.build_info.contract_crate.name.to_string();
        OutputContractConfig {
            default_contract_config_name: default_contract_config_name.clone(),
            contracts: vec![OutputContract {
                main: true,
                external_view: false,
                config_name: default_contract_config_name.clone(),
                public_name: default_contract_config_name,
                abi: original_abi.clone(),
                cargo_toml_contents_cache: None,
            }],
        }
    }

    pub fn load_from_file_or_default<P: AsRef<Path>>(path: P, original_abi: &ContractAbi) -> Self {
        match fs::read_to_string(path.as_ref()) {
            Ok(s) => {
                let config_serde: MultiContractConfigSerde = toml::from_str(s.as_str())
                    .unwrap_or_else(|error| panic!("error parsing multicontract.toml: {}", error));
                Self::load_from_config(&config_serde, original_abi)
            },
            Err(_) => Self::default_config(original_abi),
        }
    }
}
