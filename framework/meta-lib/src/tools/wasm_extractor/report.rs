use std::collections::HashMap;

use super::code_report::CodeReport;

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct WasmReport {
    pub imports: Vec<String>,
    pub memory_grow_flag: bool,
    pub ei_check: bool,
    pub code: CodeReport,
    pub forbidden_opcodes: HashMap<String, Vec<String>>,
}
