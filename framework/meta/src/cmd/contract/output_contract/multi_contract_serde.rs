use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MultiContractConfigSerde {
    #[serde(default)]
    pub settings: MultiContractGeneralSettingsSerde,
    #[serde(default)]
    pub contracts: HashMap<String, OutputContractSerde>,
    #[serde(default)]
    #[serde(rename = "labels-for-contracts")]
    pub labels_for_contracts: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct OutputContractSerde {
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
}

#[derive(Deserialize, Default, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub main: Option<String>,
}
