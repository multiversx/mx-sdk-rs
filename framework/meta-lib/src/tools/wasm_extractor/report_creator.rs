use std::path::PathBuf;

use crate::tools::panic_report::PanicReport;

#[derive(PartialEq, Eq, Debug)]
pub struct ReportCreator {
    pub path: PathBuf,
    pub has_allocator: bool,
    pub has_panic: PanicReport,
    pub forbidden_opcodes: Vec<String>,
}

impl ReportCreator {}

impl Default for ReportCreator {
    fn default() -> Self {
        ReportCreator {
            path: PathBuf::from(""),
            has_allocator: false,
            has_panic: PanicReport::None,
            forbidden_opcodes: Vec::new(),
        }
    }
}
