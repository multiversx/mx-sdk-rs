use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct MultiContract {
    pub settings: MultiContractGeneralSettings,
    pub contracts: HashMap<String, MultiContractInstance>,
    pub labels: HashMap<String, MultiContractTargetLabel>,
}

#[derive(Deserialize, Debug)]
pub struct MultiContractInstance {
    pub external_view: Option<bool>,
    pub wasm_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MultiContractTargetLabel(pub Vec<String>);

#[derive(Deserialize, Debug)]
pub struct MultiContractGeneralSettings {
    pub default: String,
}
