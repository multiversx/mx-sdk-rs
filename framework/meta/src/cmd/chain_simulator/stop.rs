use crate::cmd::chain_simulator::error::{ChainSimulatorError, DOCKER_CMD, SIMULATOR_IMAGE};
use colored::Colorize;
use std::process::Command;

pub fn stop_and_check() -> Result<(), ChainSimulatorError> {
    println!("{}", "Attempting to close the Chain Simulator...".yellow());

    let output = Command::new(DOCKER_CMD)
        .args(["ps", "-q", "--filter", format!("ancestor={SIMULATOR_IMAGE}").as_str()])
        .output()
        .map_err(|e| {
            ChainSimulatorError::CommandFailed(format!(
                "Failed to execute `{DOCKER_CMD} ps` to find a running Chain Simulator container. Cause: {e:#?}"
            ))
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ChainSimulatorError::CommandFailed(stderr.to_string()));
    }

    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if container_id.is_empty() {
        return Err(ChainSimulatorError::ContainerNotRunning);
    }

    let stop_output = Command::new(DOCKER_CMD)
        .args(["stop", &container_id])
        .output()
        .map_err(|e| {
            ChainSimulatorError::CommandFailed(format!(
                "Failed to execute `{DOCKER_CMD} stop {container_id}`. Cause: {e:#?}"
            ))
        })?;

    if stop_output.status.success() {
        println!("{}", "Successfully stopped the Chain Simulator.".green());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&stop_output.stderr);
        Err(ChainSimulatorError::CommandFailed(stderr.to_string()))
    }
}
