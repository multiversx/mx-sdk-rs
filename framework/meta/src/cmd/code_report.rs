pub mod compare;
pub mod generate_report;
pub mod render_code_report;

use generate_report::run_code_report;

use crate::cli::{CodeReportArgs, OutputFormat};

pub fn code_report(args: &CodeReportArgs) {
    let path: &str = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    run_code_report(
        path,
        args.output.to_str().unwrap(),
        args.format.as_ref().unwrap_or(&OutputFormat::default()),
        args.compare.clone().unwrap_or_default().to_str().unwrap(),
    );
}
