
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MultiContract{
    pub default: String,
    pub labels: HashMap<String, WasmLabel>,
}

#[derive(Deserialize)]
pub struct WasmLabel{
    pub wasm: String,
    pub endpoints: Vec<String>,
}
