mod cargo;
mod error;
mod llvm_cov;
mod render;
mod run;
mod util;

use crate::{
    cli::{OutputFormat, TestCoverageArgs},
    cmd::test_coverage::{cargo::get_workspace_root, run::run_test_coverage},
};
use std::process;

pub fn test_coverage(args: &TestCoverageArgs) {
    let root_path = get_workspace_root().unwrap();
    if let Err(err) = run_test_coverage(
        &root_path,
        &args.output,
        args.format.as_ref().unwrap_or(&OutputFormat::default()),
        &args.ignore_filename_regex,
    ) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
