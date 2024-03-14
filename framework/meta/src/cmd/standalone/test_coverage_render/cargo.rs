use crate::cmd::standalone::test_coverage_render::error::TestCoverageRenderError;
use std::process::Command;

pub fn get_workspace_root() -> Result<String, TestCoverageRenderError> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version=1"])
        .output()
        .map_err(|e| TestCoverageRenderError::Cargo(format!("{}", e)))?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| TestCoverageRenderError::Cargo("invalid manifest data".into()))?;

    let Some(workspace_root) = metadata["workspace_root"].as_str() else {
        return Err(TestCoverageRenderError::Cargo(
            "invalid manifest data".into(),
        ));
    };

    Ok(workspace_root.to_owned())
}
