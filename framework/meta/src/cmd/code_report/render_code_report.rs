use std::{
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
};

use multiversx_sc_meta_lib::code_report_json::CodeReportJson;

fn writeln_output_str<S: AsRef<str>>(output: &mut String, input: S) {
    output.write_str(&format!("{}\n", input.as_ref())).ok();
}

pub fn render_report(output: &mut String, reports: &Vec<CodeReportJson>, compared_path_file: &str) {
    render_header(output);

    if compared_path_file.is_empty() {
        render_reports(output, reports);
    } else {
        render_and_compare(output, reports, compared_path_file);
    }
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
                "| {} | {} | {} | {} |",
                report.path.split('/').last().expect("no output path"),
                report.size,
                report.has_allocator,
                report.has_panic
            ),
        );
    }
}

fn render_and_compare(
    output: &mut String,
    reports: &Vec<CodeReportJson>,
    compared_path_file: &str,
) {
    let compared_file = File::open(compared_path_file).unwrap_or_else(|_| {
        panic!(
            "Failed to open compared file at path: {}",
            compared_path_file
        )
    });

    let mut compared_reports = Vec::new();
    if compared_path_file.ends_with("md") {
        compared_reports = parse_into_code_report_json(compared_file);
    } else {
    }

    for report in reports.iter() {
        let path: String = report
            .path
            .split('/')
            .last()
            .expect("no output path")
            .to_owned();
        if compared_reports.is_empty() {
            writeln_output_str(
                output,
                format!(
                    "| {} | {} | {} | {} |",
                    path, report.size, report.has_allocator, report.has_panic
                ),
            );
            continue;
        }

        if let Some(compared_report) = find_report_by_path(&compared_reports, &path) {
            print_compared_output(output, report, compared_report);
        }
    }
}

fn parse_into_code_report_json(compared_file: File) -> Vec<CodeReportJson> {
    let reader = BufReader::new(compared_file);

    let mut lines = reader.lines().skip(2);

    let mut compared_reports: Vec<CodeReportJson> = Vec::new();

    while let Some(line) = lines.next() {
        match line {
            Ok(l) => {
                let columns: Vec<String> = l
                    .split('|')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if columns.len() == 4 {
                    compared_reports.push(CodeReportJson {
                        path: columns[0].to_owned(),
                        size: columns[1].parse::<usize>().unwrap(),
                        has_allocator: columns[2].parse::<bool>().unwrap(),
                        has_panic: columns[3].to_owned(),
                    })
                }
            },
            Err(_) => return compared_reports,
        }
    }

    compared_reports
}

fn find_report_by_path<'a>(
    reports: &'a Vec<CodeReportJson>,
    contract_path: &'a String,
) -> Option<&'a CodeReportJson> {
    for report in reports {
        if report.path == *contract_path {
            return Some(report);
        }
    }

    None
}

fn print_compared_output(
    output: &mut String,
    report: &CodeReportJson,
    compared_report: &CodeReportJson,
) {
    let mut size_report;
    if report.size == compared_report.size {
        size_report = format!("{}", report.size);
    } else {
        size_report = format!("{} :arrow-right: {}", compared_report.size, report.size);
        if report.size > compared_report.size {
            size_report = format!("{size_report} :red-circle:");
        } else if report.size < compared_report.size {
            size_report = format!("{size_report} :green-circle:");
        }
    }

    let mut has_allocator_report;
    if report.has_allocator == compared_report.has_allocator {
        has_allocator_report = format!("{}", report.has_allocator);
    } else {
        has_allocator_report = format!(
            "{} :arrow-right: {}",
            compared_report.has_allocator, report.has_allocator
        );

        if !report.has_allocator {
            has_allocator_report = format!("{has_allocator_report} :green-circle:");
        } else {
            has_allocator_report = format!("{has_allocator_report} :red-circle:");
        }
    }

    let mut has_panic_report;
    if report.has_panic == compared_report.has_panic {
        has_panic_report = format!("{}", report.has_allocator);
    } else {
        has_panic_report = format!(
            "{} :arrow-right: {}",
            compared_report.has_panic, report.has_panic
        );

        if report.has_panic == "none" {
            has_panic_report = format!("{has_panic_report} :green-circle:");
        }
    }

    writeln_output_str(
        output,
        format!(
            "| {} | {} | {} | {} |",
            report.path.split('/').last().expect("no output path"),
            size_report,
            has_allocator_report,
            has_panic_report
        ),
    );
}
