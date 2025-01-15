use super::panic_report::PanicReport;

pub struct ReportCreator {
    pub path: String,
    pub has_allocator: bool,
    pub has_panic: PanicReport,
}

impl ReportCreator {}

impl Default for ReportCreator {
    fn default() -> Self {
        ReportCreator {
            path: String::new(),
            has_allocator: false,
            has_panic: PanicReport::None,
        }
    }
}
