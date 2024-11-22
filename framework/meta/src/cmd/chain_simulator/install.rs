use crate::cmd::chain_simulator::error::{ChainSimulatorError, DOCKER_CMD, SIMULATOR_IMAGE};
use colored::Colorize;
use std::process::Command;

pub fn install_and_check() -> Result<(), ChainSimulatorError> {
    println!(
        "{}",
        "Attempting to install prerequisites for the Chain Simulator...".yellow()
    );

    if Command::new(DOCKER_CMD).arg("--version").output().is_err() {
        return Err(ChainSimulatorError::DockerNotInstalled);
    }

    let output = Command::new(DOCKER_CMD)
        .args(["image", "pull", SIMULATOR_IMAGE])
        .output()
        .map_err(|e| {
            ChainSimulatorError::CommandFailed(format!(
                "Failed to execute `{DOCKER_CMD} image pull {SIMULATOR_IMAGE}`. Cause: {e:#?}"
            ))
        })?;

    if output.status.success() {
        println!(
            "{}",
            "Successfully pulled the latest Chain Simulator image.".green()
        );
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(ChainSimulatorError::CommandFailed(stderr.to_string()))
    }
}
