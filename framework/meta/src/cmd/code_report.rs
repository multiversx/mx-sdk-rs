pub mod compare;
pub mod generate_report;
pub mod render_code_report;

use generate_report::{compare_report, convert_report, create_report};

use crate::cli::{CodeReportAction, CodeReportArgs};

pub fn report(args: &CodeReportArgs) {
    match &args.command {
        CodeReportAction::Compile(compile_args) => create_report(compile_args),
        CodeReportAction::Compare(compare_args) => compare_report(compare_args),
        CodeReportAction::Convert(convert_args) => convert_report(convert_args),
    }
}
