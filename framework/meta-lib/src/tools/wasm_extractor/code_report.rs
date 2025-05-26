use std::path::PathBuf;

use crate::tools::panic_report::PanicReport;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CodeReport {
    pub path: PathBuf,
    pub has_allocator: bool,
    pub has_panic: PanicReport,
}

impl CodeReport {}

impl Default for CodeReport {
    fn default() -> Self {
        CodeReport {
            path: PathBuf::from(""),
            has_allocator: false,
            has_panic: PanicReport::None,
        }
    }
}
