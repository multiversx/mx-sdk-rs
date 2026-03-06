use std::collections::{BTreeMap, BTreeSet};

use super::{EndpointName, code_report::CodeReport};

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct WasmReport {
    pub imports: Vec<String>,
    pub memory_grow_flag: bool,
    pub ei_check: bool,
    pub code: CodeReport,
    pub forbidden_opcodes: BTreeMap<EndpointName, BTreeSet<String>>,
}

impl WasmReport {
    pub fn add_forbidden_opcode_accessible_from_endpoint(
        &mut self,
        endpoint_name: EndpointName,
        opcode_name: String,
    ) {
        self.forbidden_opcodes
            .entry(endpoint_name)
            .or_default()
            .insert(opcode_name);
    }
}
