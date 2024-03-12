mod llvm_cov;
mod renderer;
mod cargo;

use std::env;
use anyhow::{bail, Result};

use llvm_cov::parse_llvm_cov_output;
use renderer::render_coverage;

fn main() -> Result<()> {
    let mut args = env::args();

    if args.len() < 2 {
        bail!("Usage: coverage-renderer <input_path>");
    }

    let root = cargo::get_workspace_root()?;

    let input_path = args.nth(1).unwrap();
    let input = std::fs::read_to_string(input_path)?;

    let coverage = parse_llvm_cov_output(&input)?;
    
    render_coverage(&coverage, &root);

    Ok(())
}
