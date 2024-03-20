use std::process::Command;

use anyhow::{bail, Result};

pub fn get_workspace_root() -> Result<String> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version=1"])
        .output()?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;

    let Some(workspace_root) = metadata["workspace_root"].as_str() else {
        bail!("Failed to get workspace root");
    };

    Ok(workspace_root.to_owned())
}
