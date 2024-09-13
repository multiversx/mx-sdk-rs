pub const WITH_MESSAGE: &str = "with message";
pub const WITHOUT_MESSAGE: &str = "without message";

pub struct ReportCreator {
    pub path: String,
    pub has_allocator: bool,
    pub has_panic: String,
}

impl ReportCreator {}

impl Default for ReportCreator {
    fn default() -> Self {
        ReportCreator {
            path: String::new(),
            has_allocator: false,
            has_panic: "none".to_string(),
        }
    }
}
