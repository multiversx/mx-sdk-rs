use serde::Deserialize;
use std::collections::HashMap;

use crate::ei::EIVersion;

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
    pub features: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub main: Option<String>,
}

pub fn parse_check_ei(ei: &Option<String>) -> Option<EIVersion> {
    if let Some(ei_name) = ei {
        if ei_name == "ignore" {
            None
        } else {
            let ei_version = EIVersion::from_name(ei_name)
                .unwrap_or_else(|| panic!("invalid EI version: {ei_name}"));
            Some(ei_version)
        }
    } else {
        Some(EIVersion::default())
    }
}
