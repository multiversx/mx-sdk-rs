use anyhow::{bail, Result};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Coverage {
    pub files: Vec<FileSummary>,
    pub totals: Summary,
}

#[derive(Deserialize)]
pub struct SummaryItem {
    pub count: u64,
    pub covered: u64,
    pub percent: f64,
}

#[derive(Deserialize)]
pub struct Summary {
    pub functions: SummaryItem,
    pub lines: SummaryItem,
    pub instantiations: SummaryItem,
    pub regions: SummaryItem,
}

#[derive(Deserialize)]
pub struct FileSummary {
    pub filename: String,
    pub summary: Summary,
}

pub fn parse_llvm_cov_output(output: &str) -> Result<Coverage> {
    let llvm_cov_output: Value = serde_json::from_str(output)?;
    let Some(coverage) = llvm_cov_output.get("data").and_then(|data| data.get(0)) else {
        bail!("Invalid llvm-cov output");
    };

    let coverage = serde_json::from_value(coverage.to_owned())?;
    Ok(coverage)
}
