pub mod code_report;
pub mod render_code_report;

use code_report::run_code_report;

use crate::cli::CodeReportArgs;

pub fn code_report(args: &CodeReportArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    run_code_report(path);
}
