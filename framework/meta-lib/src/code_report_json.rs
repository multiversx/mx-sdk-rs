use serde::{Deserialize, Serialize};

use crate::tools::report_creator::ReportCreator;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeReportJson {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub size: usize,

    #[serde(default)]
    pub has_allocator: bool,

    #[serde(default)]
    pub has_panic: String,
}

impl CodeReportJson {
    pub fn new(report: &ReportCreator, size: usize) -> CodeReportJson {
        CodeReportJson {
            path: report.path.clone(),
            size,
            has_allocator: report.has_allocator,
            has_panic: report.has_panic.to_string(),
        }
    }
}
