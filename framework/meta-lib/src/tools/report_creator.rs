use std::path::PathBuf;

use super::panic_report::PanicReport;

pub struct ReportCreator {
    pub path: PathBuf,
    pub has_allocator: bool,
    pub has_panic: PanicReport,
}

impl ReportCreator {}

impl Default for ReportCreator {
    fn default() -> Self {
        ReportCreator {
            path: PathBuf::from(""),
            has_allocator: false,
            has_panic: PanicReport::None,
        }
    }
}
