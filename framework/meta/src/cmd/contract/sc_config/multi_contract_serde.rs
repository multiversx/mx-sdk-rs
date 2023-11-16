use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MultiContractConfigSerde {
    #[serde(default)]
    pub settings: MultiContractGeneralSettingsSerde,
    #[serde(default)]
    pub contracts: HashMap<String, ContractVariantSerde>,
    #[serde(default)]
    #[serde(rename = "labels-for-contracts")]
    pub labels_for_contracts: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct ContractVariantSerde {
    pub name: Option<String>,

    #[serde(default)]
    #[serde(rename = "add-unlabelled")]
    pub add_unlabelled: Option<bool>,

    #[serde(default)]
    #[serde(rename = "add-labels")]
    pub add_labels: Vec<String>,

    #[serde(default)]
    #[serde(rename = "add-endpoints")]
    pub add_endpoints: Vec<String>,

    #[serde(default)]
    #[serde(rename = "external-view")]
    pub external_view: Option<bool>,

    #[serde(default)]
    #[serde(rename = "panic-message")]
    pub panic_message: Option<bool>,

    #[serde(default)]
    pub ei: Option<String>,

    #[serde(default)]
    pub allocator: Option<String>,

    #[serde(default)]
    #[serde(rename = "stack-size")]
    pub stack_size: Option<String>,

    #[serde(default)]
    pub features: Vec<String>,

    #[serde(default)]
    pub kill_legacy_callback: bool,

    #[serde(default)]
    pub contract_variant_profile: Option<ContractVariantProfile>,
}

#[derive(Deserialize, Default, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub main: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ContractVariantProfile {
    pub codegen_units: u8,
    pub opt_level: String,
    pub lto: bool,
    pub debug: bool,
    pub panic: String,
}

impl Default for ContractVariantProfile {
    fn default() -> ContractVariantProfile {
        ContractVariantProfile {
            codegen_units: 1u8,
            opt_level: "z".to_owned(),
            lto: true,
            debug: false,
            panic: "abort".to_owned(),
        }
    }
}
