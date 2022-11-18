use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MultiContractSerde {
    pub settings: MultiContractGeneralSettingsSerde,
    pub contracts: HashMap<String, MultiContractInstanceSerde>,
    pub labels: HashMap<String, MultiContractTargetLabelSerde>,
}

#[derive(Deserialize, Debug)]
pub struct MultiContractInstanceSerde {
    pub external_view: Option<bool>,
    pub wasm_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MultiContractTargetLabelSerde(pub Vec<String>);

#[derive(Deserialize, Debug)]
pub struct MultiContractGeneralSettingsSerde {
    pub default: String,
}
