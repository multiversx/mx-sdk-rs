use crate::cmd::chain_simulator::error::{
    ChainSimulatorError, DEFAULT_PORT, DOCKER_CMD, SIMULATOR_IMAGE,
};
use colored::Colorize;
use std::process::Command;

pub fn start_and_check() -> Result<(), ChainSimulatorError> {
    println!("{}", "Attempting to start the Chain Simulator...".yellow());

    let output = Command::new(DOCKER_CMD)
        .args(["run", "-p", DEFAULT_PORT, SIMULATOR_IMAGE])
        .output()
        .map_err(|e| {
            ChainSimulatorError::CommandFailed(format!(
                "Failed to execute `{DOCKER_CMD} run -p {DEFAULT_PORT} {SIMULATOR_IMAGE}`. Cause: {e:#?}"
            ))
        })?;

    if output.status.success() {
        println!("{}", "Successfully started the Chain Simulator.".green());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(ChainSimulatorError::CommandFailed(stderr.to_string()))
    }
}
