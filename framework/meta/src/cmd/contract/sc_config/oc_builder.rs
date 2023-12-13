use multiversx_sc::abi::{ContractAbi, EndpointAbi};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use super::{
    oc_global_config::SC_CONFIG_FILE_NAMES,
    oc_settings::{parse_allocator, parse_check_ei, parse_stack_size},
    ContractVariant, ContractVariantProfile, ContractVariantSerde, ContractVariantSettings,
    MultiContractConfigSerde, ScConfig,
};

/// Temporary structure, to help create instances of `ContractVariant`. Not publicly exposed.
struct ContractVariantBuilder {
    pub contract_id: String,
    pub explicit_name: String,
    pub add_unlabelled: bool,
    pub add_labels: BTreeSet<String>,
    pub add_endpoints: BTreeSet<String>,
    pub collected_endpoints: Vec<EndpointAbi>,
    endpoint_names: HashSet<String>, // help keep endpoints unique
    pub settings: ContractVariantSettings,
}

impl Default for ContractVariantBuilder {
    fn default() -> Self {
        Self {
            contract_id: Default::default(),
            explicit_name: Default::default(),
            add_unlabelled: true,
            add_labels: Default::default(),
            add_endpoints: Default::default(),
            collected_endpoints: Default::default(),
            endpoint_names: Default::default(),
            settings: Default::default(),
        }
    }
}

impl ContractVariantBuilder {
    fn new(id: String) -> Self {
        ContractVariantBuilder {
            contract_id: id.clone(),
            explicit_name: id,
            ..Default::default()
        }
    }

    fn map_from_config(kvp: (&String, &ContractVariantSerde)) -> (String, ContractVariantBuilder) {
        let (contract_id, cms) = kvp;
        let external_view = cms.external_view.unwrap_or_default();
        let mut collected_endpoints = Vec::new();
        if external_view {
            collected_endpoints.push(
                multiversx_sc::external_view_contract::external_view_contract_constructor_abi(),
            )
        }
        let default = ContractVariantBuilder::default();
        (
            contract_id.clone(),
            ContractVariantBuilder {
                contract_id: contract_id.clone(),
                explicit_name: cms.name.clone().unwrap_or(default.explicit_name),
                add_unlabelled: cms.add_unlabelled.unwrap_or(default.add_unlabelled),
                add_labels: cms.add_labels.iter().cloned().collect(),
                add_endpoints: cms.add_endpoints.iter().cloned().collect(),
                collected_endpoints,
                settings: ContractVariantSettings {
                    external_view: cms.external_view.unwrap_or(default.settings.external_view),
                    panic_message: cms.panic_message.unwrap_or(default.settings.panic_message),
                    check_ei: parse_check_ei(&cms.ei),
                    allocator: parse_allocator(&cms.allocator),
                    stack_size: parse_stack_size(&cms.stack_size),
                    features: cms.features.clone(),
                    kill_legacy_callback: cms.kill_legacy_callback,
                    profile: ContractVariantProfile::from_serde(&cms.profile),
                },
                ..default
            },
        )
    }

    fn wasm_name(&self) -> &String {
        if !self.explicit_name.is_empty() {
            &self.explicit_name
        } else {
            &self.contract_id
        }
    }

    fn collect_endpoint(&mut self, endpoint_abi: &EndpointAbi) {
        if !self.endpoint_names.contains(&endpoint_abi.name) {
            self.endpoint_names.insert(endpoint_abi.name.clone());
            self.collected_endpoints.push(endpoint_abi.clone());
        }
    }
}

fn process_labels_for_contracts(
    contract_builders: &mut HashMap<String, ContractVariantBuilder>,
    labels_for_contracts: &HashMap<String, Vec<String>>,
) {
    for (label, targets) in labels_for_contracts {
        for target in targets {
            contract_builders
                .entry(target.clone())
                .or_insert_with(|| ContractVariantBuilder::new(target.clone()))
                .add_labels
                .insert(label.clone());
        }
    }
}

fn endpoint_unlabelled(endpoint_abi: &EndpointAbi) -> bool {
    endpoint_abi.labels.is_empty()
}

fn endpoint_matches_labels(endpoint_abi: &EndpointAbi, labels: &BTreeSet<String>) -> bool {
    endpoint_abi
        .labels
        .iter()
        .any(|endpoint_label| labels.contains(endpoint_label))
}

fn collect_unlabelled_endpoints(
    contract_builders: &mut HashMap<String, ContractVariantBuilder>,
    original_abi: &ContractAbi,
) {
    for builder in contract_builders.values_mut() {
        if builder.add_unlabelled {
            for endpoint_abi in original_abi.iter_all_exports() {
                if endpoint_unlabelled(endpoint_abi) {
                    builder.collect_endpoint(endpoint_abi);
                }
            }
        }
    }
}

fn collect_labelled_endpoints(
    contract_builders: &mut HashMap<String, ContractVariantBuilder>,
    original_abi: &ContractAbi,
) {
    for builder in contract_builders.values_mut() {
        for endpoint_abi in original_abi.iter_all_exports() {
            if endpoint_matches_labels(endpoint_abi, &builder.add_labels) {
                builder.collect_endpoint(endpoint_abi);
            }
        }
    }
}

fn collect_add_endpoints(
    contract_builders: &mut HashMap<String, ContractVariantBuilder>,
    original_abi: &ContractAbi,
) {
    for builder in contract_builders.values_mut() {
        for endpoint_abi in original_abi.iter_all_exports() {
            if builder.add_endpoints.contains(&endpoint_abi.name) {
                builder.collect_endpoint(endpoint_abi);
            }
        }
    }
}

fn build_contract_abi(builder: ContractVariantBuilder, original_abi: &ContractAbi) -> ContractAbi {
    let mut constructors = Vec::new();
    let mut endpoints = Vec::new();
    let mut promise_callbacks = Vec::new();
    for endpoint_abi in builder.collected_endpoints {
        match endpoint_abi.endpoint_type {
            multiversx_sc::abi::EndpointTypeAbi::Init => constructors.push(endpoint_abi),
            multiversx_sc::abi::EndpointTypeAbi::Endpoint => endpoints.push(endpoint_abi),
            multiversx_sc::abi::EndpointTypeAbi::PromisesCallback => {
                promise_callbacks.push(endpoint_abi)
            },
        }
    }
    let has_callback = original_abi.has_callback
        && !builder.settings.external_view
        && !builder.settings.kill_legacy_callback;
    ContractAbi {
        build_info: original_abi.build_info.clone(),
        docs: original_abi.docs.clone(),
        name: original_abi.name.clone(),
        constructors,
        endpoints,
        promise_callbacks,
        events: original_abi.events.clone(),
        has_callback,
        type_descriptions: original_abi.type_descriptions.clone(),
        esdt_attributes: original_abi.esdt_attributes.clone(),
    }
}

fn default_wasm_crate_name(contract_name: &str) -> String {
    format!("{contract_name}-wasm")
}

fn build_contract(builder: ContractVariantBuilder, original_abi: &ContractAbi) -> ContractVariant {
    let contract_name = builder.wasm_name().clone();
    let wasm_crate_name = default_wasm_crate_name(&contract_name);
    ContractVariant {
        main: false,
        settings: builder.settings.clone(),
        contract_id: builder.contract_id.clone(),
        contract_name,
        wasm_crate_name,
        abi: build_contract_abi(builder, original_abi),
    }
}

fn set_main_contract_flag(
    contracts: &mut [ContractVariant],
    default_contract_config_name_opt: &Option<String>,
) {
    if let Some(default_contract_config_name) = default_contract_config_name_opt {
        for contract in contracts.iter_mut() {
            if &contract.contract_id == default_contract_config_name {
                contract.main = true;
                return;
            }
        }

        panic!(
            "Could not find default contract '{}' among the output contracts. Available contracts are: {:?}",
            default_contract_config_name,
            contracts.iter().map(|contract| &contract.contract_id).collect::<Vec<_>>(),
        )
    } else {
        let first_contract = contracts.get_mut(0).unwrap_or_else(|| {
            panic!("Cannot set default contract because no optput contract was specified.")
        });
        first_contract.main = true;
    }
}

fn validate_contract_variants(contracts: &[ContractVariant]) {
    for contract in contracts {
        if contract.main {
            assert!(
                contract.settings.features.is_empty(),
                "features not supported for main contract"
            );
        }
    }
}

impl ScConfig {
    /// Assembles an `ContractVariantConfig` from a raw config object that was loaded via Serde.
    ///
    /// In most cases the config will be loaded from a .toml file, use `load_from_file` for that.
    pub fn load_from_config(config: &MultiContractConfigSerde, original_abi: &ContractAbi) -> Self {
        let mut contract_builders: HashMap<String, ContractVariantBuilder> = config
            .contracts
            .iter()
            .map(ContractVariantBuilder::map_from_config)
            .collect();
        collect_unlabelled_endpoints(&mut contract_builders, original_abi);
        collect_labelled_endpoints(&mut contract_builders, original_abi);
        collect_add_endpoints(&mut contract_builders, original_abi);
        process_labels_for_contracts(&mut contract_builders, &config.labels_for_contracts);
        let mut contracts: Vec<ContractVariant> = contract_builders
            .into_values()
            .map(|builder| build_contract(builder, original_abi))
            .collect();
        set_main_contract_flag(&mut contracts, &config.settings.main);
        validate_contract_variants(&contracts);
        ScConfig {
            default_contract_config_name: config.settings.main.clone().unwrap_or_default(),
            contracts,
        }
    }

    /// Provides the config for the cases where no `multicontract.toml` file is available.
    ///
    /// The default configuration contains a single main contract, with all endpoints.
    pub fn default_config(original_abi: &ContractAbi) -> Self {
        let default_contract_config_name = original_abi.build_info.contract_crate.name.to_string();
        let wasm_crate_name = default_wasm_crate_name(&default_contract_config_name);
        ScConfig {
            default_contract_config_name: default_contract_config_name.clone(),
            contracts: vec![ContractVariant {
                main: true,
                settings: ContractVariantSettings::default(),
                contract_id: default_contract_config_name.clone(),
                contract_name: default_contract_config_name,
                wasm_crate_name,
                abi: original_abi.clone(),
            }],
        }
    }

    /// Loads a contract configuration from file. Will return `None` if the file is not found.
    pub fn load_from_file<P: AsRef<Path>>(path: P, original_abi: &ContractAbi) -> Option<Self> {
        match fs::read_to_string(path.as_ref()) {
            Ok(s) => {
                let config_serde: MultiContractConfigSerde = toml::from_str(s.as_str())
                    .unwrap_or_else(|error| panic!("error parsing multicontract.toml: {error}"));
                Some(Self::load_from_config(&config_serde, original_abi))
            },
            Err(_) => None,
        }
    }

    /// The standard way of loading a `multicontract.toml` configuration: read the file if present, use the default config otherwise.
    pub fn load_from_files_or_default<I, P>(paths: I, original_abi: &ContractAbi) -> Self
    where
        P: AsRef<Path>,
        I: Iterator<Item = P>,
    {
        for path in paths {
            if let Some(config) = Self::load_from_file(path.as_ref(), original_abi) {
                return config;
            }
        }

        Self::default_config(original_abi)
    }

    /// The standard way of loading a `multicontract.toml` configuration: read the file if present, use the default config otherwise.
    pub fn load_from_crate_or_default<P>(contract_crate_path: P, original_abi: &ContractAbi) -> Self
    where
        P: AsRef<Path>,
    {
        Self::load_from_files_or_default(
            SC_CONFIG_FILE_NAMES
                .iter()
                .map(|name| PathBuf::from(contract_crate_path.as_ref()).join(name)),
            original_abi,
        )
    }
}
