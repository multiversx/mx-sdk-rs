use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MultiContractConfigSerde {
    pub settings: MultiContractGeneralSettingsSerde,
    pub contracts: HashMap<String, ContractMetadataSerde>,
    pub labels: HashMap<String, MultiContractTargetLabelSerde>,
}

#[derive(Deserialize, Debug)]
pub struct ContractMetadataSerde {
    pub external_view: Option<bool>,
    pub wasm_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MultiContractTargetLabelSerde(pub Vec<String>);

#[derive(Deserialize, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub main: Option<String>,
}
