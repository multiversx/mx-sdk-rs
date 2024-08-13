use std::fmt::Display;

pub struct CodeReportRender<'a> {
    pub file: Option<&'a mut dyn std::io::Write>,
    pub compared_reports: &'a [CodeReportJson],
    pub reports: &'a [CodeReportJson],
}

use multiversx_sc_meta_lib::code_report_json::CodeReportJson;

use super::compare::{
    allocator_status_after_comparing, panic_status_after_comparing, size_status_after_comparing,
};

impl<'a> CodeReportRender<'a> {
    pub fn new(
        file: &'a mut dyn std::io::Write,
        compared_reports: &'a [CodeReportJson],
        reports: &'a [CodeReportJson],
    ) -> Self {
        Self {
            file: Some(file),
            compared_reports,
            reports,
        }
    }

    pub fn new_without_compare(
        file: &'a mut dyn std::io::Write,
        reports: &'a [CodeReportJson],
    ) -> Self {
        Self {
            file: Some(file),
            compared_reports: &[],
            reports,
        }
    }

    pub fn render_report(&mut self) {
        self.render_header();

        self.render_reports();
    }

    pub fn compare_reports(&mut self) {
        self.render_header();

        if self.compared_reports.is_empty() {
            self.render_reports();
            self.writeln("\n:warning: Could not download the report for the base branch. Displaying only the report for the current branch. :warning:");
        } else {
            self.render_report_and_compare();
        }
    }

    fn writeln(&mut self, s: impl Display) {
        let file = self.file.as_mut().unwrap();
        file.write_all(s.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    fn write_report_for_contract(
        &mut self,
        path: &String,
        size: &String,
        has_allocator: &String,
        has_panic: &String,
    ) {
        self.writeln(format!(
            "| {} | {} | {} | {} |",
            path.split('/').last().unwrap_or_else(|| path),
            size,
            has_allocator,
            has_panic
        ));
    }

    fn render_header(&mut self) {
        self.writeln("| Path                                                         |                                     size |                  has-allocator |                     has-format |");
        self.writeln("| :-- | --: | --: | --: |");
    }

    fn render_reports(&mut self) {
        for report in self.reports {
            self.write_report_for_contract(
                &report.path,
                &report.size.to_string(),
                &report.has_allocator.to_string(),
                &report.has_panic,
            );
        }
    }

    fn render_report_and_compare(&mut self) {
        for report in self.reports.iter() {
            if let Some(compared_report) = self
                .compared_reports
                .iter()
                .find(|cr| cr.path == report.path)
            {
                self.print_compared_output(report, compared_report);
            }
        }
    }

    fn print_compared_output(&mut self, report: &CodeReportJson, compared_report: &CodeReportJson) {
        let size_report = size_status_after_comparing(report.size, compared_report.size);

        let has_allocator_report =
            allocator_status_after_comparing(report.has_allocator, compared_report.has_allocator);

        let has_panic_report =
            panic_status_after_comparing(&report.has_panic, &compared_report.has_panic);

        self.write_report_for_contract(
            &report.path,
            &size_report,
            &has_allocator_report,
            &has_panic_report,
        );
    }
}
