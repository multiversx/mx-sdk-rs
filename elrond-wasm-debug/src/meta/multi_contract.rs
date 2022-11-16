
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct MultiContract{
    pub default: String,
    pub labels: HashMap<String, WasmLabel>,
}

#[derive(Deserialize)]
pub(crate) struct WasmLabel{
    pub name:String,
    pub endpoints: Vec<String>,
}
