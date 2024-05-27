use crate::cmd::test_coverage::error::TestCoverageError;
use serde::Deserialize;
use std::process::Command;

pub fn get_workspace_root() -> Result<String, TestCoverageError> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version=1"])
        .output()
        .map_err(|e| TestCoverageError::Cargo(format!("{}", e)))?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| TestCoverageError::Cargo("invalid manifest data".into()))?;

    let Some(workspace_root) = metadata["workspace_root"].as_str() else {
        return Err(TestCoverageError::Cargo("invalid manifest data".into()));
    };

    Ok(workspace_root.to_owned())
}

#[derive(Debug, Clone, Deserialize)]
struct PartialCompilerArtifactMessage {
    filenames: Vec<String>,
    profile: PartialCompilerArtifactProfile,
}

#[derive(Debug, Clone, Deserialize)]
struct PartialCompilerArtifactProfile {
    test: Option<bool>,
}

pub fn run_instrumented_tests(path: &str) -> Result<(), TestCoverageError> {
    let Ok(status) = Command::new("cargo")
        .current_dir(path)
        .env("RUSTFLAGS", "-C instrument-coverage")
        .args(vec!["test", "--tests"])
        .status()
    else {
        return Err(TestCoverageError::Cargo(
            "can't run instrumented tests".into(),
        ));
    };

    if !status.success() {
        return Err(TestCoverageError::Cargo(
            "can't run instrumented tests".into(),
        ));
    }

    Ok(())
}

pub fn get_instrumented_test_binaries_paths(path: &str) -> Result<Vec<String>, TestCoverageError> {
    let Ok(output) = Command::new("cargo")
        .current_dir(path)
        .env("RUSTFLAGS", "-C instrument-coverage")
        .args(vec!["test", "--tests", "--no-run", "--message-format=json"])
        .output()
    else {
        return Err(TestCoverageError::Cargo(
            "can't get test binaries paths".into(),
        ));
    };

    if !output.status.success() {
        return Err(TestCoverageError::Cargo(
            "can't get test binaries paths".into(),
        ));
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let messages = output.split('\n').collect::<Vec<_>>();

    let mut result = vec![];
    for message in messages {
        let Ok(message) = serde_json::from_str::<serde_json::Value>(message) else {
            continue;
        };

        let Some("compiler-artifact") = message.get("reason").and_then(|val| val.as_str()) else {
            continue;
        };

        let Ok(mut message) = serde_json::from_value::<PartialCompilerArtifactMessage>(message)
        else {
            continue;
        };

        let is_test = message.profile.test.unwrap_or_default();
        if !is_test {
            continue;
        }

        result.append(&mut message.filenames);
    }

    Ok(result)
}
