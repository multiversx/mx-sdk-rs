use std::io::BufRead;

use multiversx_sc_meta_lib::code_report_json::CodeReportJson;

pub(crate) fn parse_into_code_report_json(
    compared_file_reader: &mut dyn BufRead,
) -> Vec<CodeReportJson> {
    let lines = compared_file_reader.lines().skip(2);

    let mut compared_reports: Vec<CodeReportJson> = Vec::new();

    for line in lines {
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

pub(crate) fn size_status_after_comparing(size: usize, compared_size: usize) -> String {
    match size.cmp(&compared_size) {
        std::cmp::Ordering::Greater => {
            format!("{} :arrow_right: {} :red_circle:", compared_size, size)
        },
        std::cmp::Ordering::Less => {
            format!("{} :arrow_right: {} :green_circle:", compared_size, size)
        },
        std::cmp::Ordering::Equal => {
            format!("{}", size)
        },
    }
}

pub(crate) fn allocator_status_after_comparing(
    has_allocator: bool,
    compared_has_allocator: bool,
) -> String {
    if compared_has_allocator == has_allocator {
        return format!("{}", has_allocator);
    }

    let allocator_status = format!("{} :arrow-right: {}", compared_has_allocator, has_allocator);

    if !has_allocator {
        format!("{allocator_status} :green-circle:")
    } else {
        format!("{allocator_status} :red-circle:")
    }
}

pub(crate) fn panic_status_after_comparing(
    has_panic: &String,
    compared_has_panic: &String,
) -> String {
    if has_panic == compared_has_panic {
        return has_panic.to_string();
    }

    let panic_status = format!("{} :arrow-right: {}", compared_has_panic, has_panic);

    if has_panic == "none" {
        return format!("{panic_status} :green-circle:");
    }

    panic_status
}
