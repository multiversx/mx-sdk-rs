use std::fs;

use crate::{
    cli::OutputFormat,
    cmd::test_coverage::{
        cargo::{get_instrumented_test_binaries_paths, run_instrumented_tests},
        error::TestCoverageError,
        llvm_cov::{combine_instrumentation_results, export_coverage_summary},
        render::render_coverage,
        util::{
            cleanup_file, cleanup_many_files, deep_find_files_with_ext, ensure_dependencies_in_path,
        },
    },
};

pub fn run_test_coverage(
    root_path: &str,
    output_path: &str,
    output_format: &OutputFormat,
    ignore_filename_regex: &[String],
) -> Result<(), TestCoverageError> {
    ensure_dependencies_in_path()?;

    run_instrumented_tests(root_path)?;
    let test_binaries = get_instrumented_test_binaries_paths(root_path)?;

    let instrumentation_output_files = deep_find_files_with_ext(root_path, "profraw")?;

    let combined_instrumentation_output =
        combine_instrumentation_results(root_path, &instrumentation_output_files)?;

    cleanup_many_files(&instrumentation_output_files)?;

    let coverage = export_coverage_summary(
        root_path,
        &combined_instrumentation_output,
        &test_binaries,
        ignore_filename_regex,
    )?;

    cleanup_file(&combined_instrumentation_output)?;

    let mut output = String::new();

    match output_format {
        OutputFormat::Markdown => {
            render_coverage(&mut output, &coverage, root_path);
        }
        OutputFormat::Json => {
            output = serde_json::to_string_pretty(&coverage).unwrap();
        }
    };

    let Ok(_) = fs::write(output_path, output) else {
        return Err(TestCoverageError::FsError(format!(
            "failed to write to {output_path}"
        )));
    };

    Ok(())
}
