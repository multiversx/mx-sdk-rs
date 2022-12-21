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
    #[serde(rename = "external-view")]
    pub external_view: Option<bool>,
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
}

#[derive(Deserialize, Default, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub main: Option<String>,
}
