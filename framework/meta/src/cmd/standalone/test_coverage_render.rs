mod cargo;
mod error;
mod llvm_cov;
mod render;

use crate::{
    cli_args::TestCoverageRenderArgs,
    cmd::standalone::test_coverage_render::{
        cargo::get_workspace_root, error::TestCoverageRenderError, llvm_cov::parse_llvm_cov_output,
        render::render_coverage,
    },
};
use std::{fs, process};

fn run_test_coverage_renderer(
    args: &TestCoverageRenderArgs,
) -> Result<(), TestCoverageRenderError> {
    let Ok(input) = fs::read_to_string(&args.input) else {
        return Err(TestCoverageRenderError::InvalidInputPath(
            "failed to read".into(),
        ));
    };

    let root = get_workspace_root()?;

    let coverage = parse_llvm_cov_output(&input)?;

    let mut output = String::new();

    render_coverage(&mut output, &coverage, &root);

    let Ok(_) = fs::write(&args.output, output) else {
        return Err(TestCoverageRenderError::InvalidOutputPath(
            "failed to write".into(),
        ));
    };

    Ok(())
}

pub fn test_coverage_render(args: &TestCoverageRenderArgs) {
    if let Err(err) = run_test_coverage_renderer(args) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
