use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::PathBuf, process::Command};

use super::error::TestCoverageError;

const DEFAULT_IGNORE_FILENAME_REGEX: [&str; 2] = ["/.cargo/registry", "rustc/"];

#[derive(Serialize, Deserialize)]
pub struct Coverage {
    pub files: Vec<FileSummary>,
    pub totals: Summary,
}

#[derive(Serialize, Deserialize)]
pub struct SummaryItem {
    pub count: u64,
    pub covered: u64,
    pub percent: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Summary {
    pub functions: SummaryItem,
    pub lines: SummaryItem,
    pub instantiations: SummaryItem,
    pub regions: SummaryItem,
}

#[derive(Serialize, Deserialize)]
pub struct FileSummary {
    pub filename: String,
    pub summary: Summary,
}

fn parse_llvm_cov_output(output: &str) -> Result<Coverage, TestCoverageError> {
    let llvm_cov_output: Value = serde_json::from_str(output)
        .map_err(|_| TestCoverageError::LlvmCov("invalid output data".into()))?;

    let Some(coverage) = llvm_cov_output.get("data").and_then(|data| data.get(0)) else {
        return Err(TestCoverageError::LlvmCov("invalid output data".into()));
    };

    let coverage = serde_json::from_value(coverage.to_owned())
        .map_err(|_| TestCoverageError::LlvmCov("invalid output data".into()))?;

    Ok(coverage)
}

pub fn combine_instrumentation_results(
    root_dir: &str,
    profraw_files: &Vec<String>,
) -> Result<String, TestCoverageError> {
    let Ok(output) = Command::new("llvm-profdata")
        .current_dir(root_dir)
        .args(vec!["merge", "-o", "merged.profdata", "-sparse"])
        .args(profraw_files)
        .status()
    else {
        return Err(TestCoverageError::LlvmProfdata(
            "can't merge profraw files".into(),
        ));
    };

    if !output.success() {
        return Err(TestCoverageError::LlvmProfdata(
            "can't merge profraw files".into(),
        ));
    }

    let output_path = PathBuf::from(root_dir)
        .join("merged.profdata")
        .canonicalize()
        .map_err(|_| {
            TestCoverageError::FsError("can't get canonical path for merged.profdata".into())
        })?;

    Ok(output_path.to_string_lossy().to_string())
}

pub fn export_coverage_summary(
    root_dir: &str,
    profdata_file: &str,
    binary_files: &[String],
    ignore_filename_regex: &[String],
) -> Result<Coverage, TestCoverageError> {
    let objects = binary_files
        .iter()
        .flat_map(|path| vec!["-object", path.as_str()])
        .collect::<Vec<_>>();

    let mut ignore_filename_regex = ignore_filename_regex
        .iter()
        .map(|s| format!("--ignore-filename-regex={}", s))
        .collect::<Vec<_>>();

    for ignore in DEFAULT_IGNORE_FILENAME_REGEX {
        ignore_filename_regex.push(format!("--ignore-filename-regex={}", ignore));
    }

    let Ok(output) = Command::new("llvm-cov")
        .current_dir(root_dir)
        .arg("export")
        .args(&objects)
        .args(&ignore_filename_regex)
        .args([
            &format!("--instr-profile={}", profdata_file),
            "--summary-only",
            "--format=text",
        ])
        .output()
    else {
        return Err(TestCoverageError::LlvmCov(
            "can't export coverage summary.".into(),
        ));
    };

    if !output.status.success() {
        return Err(TestCoverageError::LlvmCov(
            "can't export coverage summary".into(),
        ));
    }

    let coverage = String::from_utf8_lossy(&output.stdout).to_string();
    let coverage = parse_llvm_cov_output(&coverage)?;

    Ok(coverage)
}
