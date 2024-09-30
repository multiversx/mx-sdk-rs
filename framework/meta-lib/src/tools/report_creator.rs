use super::panic_report::PanicReport;

pub struct ReportCreator {
    pub path: String,
    pub has_allocator: bool,
    pub has_panic: PanicReport,
}

impl ReportCreator {}
