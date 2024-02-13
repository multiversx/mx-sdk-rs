use serde::{Deserialize, Serialize};

use crate::{ei_check_json::EiCheckJson, tools::WasmInfo};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportInfoJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    #[serde(default)]
    pub memory_allocation_error: bool,

    #[serde(default)]
    pub is_mem_grow: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ei_check: Option<EiCheckJson>,
}

impl ReportInfoJson {
    pub fn new(wasm_info: &WasmInfo, ei_check_info: Option<EiCheckJson>) -> Self {
        let ei_check = if wasm_info.imports.is_empty() {
            None
        } else {
            ei_check_info
        };

        ReportInfoJson {
            imports: wasm_info.imports.iter().map(|i| i.to_string()).collect(),
            memory_allocation_error: wasm_info.allocator_trigger,
            is_mem_grow: wasm_info.memory_grow_flag,
            ei_check,
        }
    }
}
