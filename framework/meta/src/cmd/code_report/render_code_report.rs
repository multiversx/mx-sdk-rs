use std::fmt::Write;

use multiversx_sc_meta_lib::code_report_json::CodeReportJson;

fn writeln_output_str<S: AsRef<str>>(output: &mut String, input: S) {
    output.write_str(&format!("{}\n", input.as_ref())).ok();
}

pub fn render_report(output: &mut String, reports: &Vec<CodeReportJson>) {
    render_header(output);
    render_reports(output, reports);
}

fn render_header(output: &mut String) {
    writeln_output_str(output, "| Path                                                         |                                     size |                  has-allocator |                     has-format |");
    writeln_output_str(output, "| :-- | --: | --: | --: |");
}

fn render_reports(output: &mut String, reports: &Vec<CodeReportJson>) {
    for report in reports {
        writeln_output_str(
            output,
            format!(
                "|  - {} | {} | {} | {} |",
                report.path.split("/").last().expect("no output path"),
                report.size,
                report.has_allocator,
                report.has_panic
            ),
        );
    }
}
