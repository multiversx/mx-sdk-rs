use serde::{Deserialize, Serialize};

use crate::{code_report_json::CodeReportJson, ei_check_json::EiCheckJson, tools::WasmReport};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportInfoJson {
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub imports: Vec<String>,

    #[serde(default)]
    pub is_mem_grow: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ei_check: Option<EiCheckJson>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub forbidden_opcodes: Vec<String>,

    #[serde(default)]
    pub code_report: CodeReportJson,
}

impl ReportInfoJson {
    pub fn new(report: &WasmReport, ei_check_info: Option<EiCheckJson>, size: usize) -> Self {
        let ei_check = if report.imports.is_empty() {
            None
        } else {
            ei_check_info
        };

        ReportInfoJson {
            imports: report.imports.iter().map(|i| i.to_string()).collect(),
            is_mem_grow: report.memory_grow_flag,
            ei_check,
            forbidden_opcodes: report
                .forbidden_opcodes
                .iter()
                .map(|i| i.to_string())
                .collect(),
            code_report: CodeReportJson::new(&report.code, size),
        }
    }
}
