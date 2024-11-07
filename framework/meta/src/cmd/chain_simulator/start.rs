use crate::cmd::chain_simulator::error::{
    ChainSimulatorError, DEFAULT_PORT, DOCKER_CMD, SIMULATOR_IMAGE,
};
use colored::Colorize;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};

pub fn start_and_check() -> Result<(), ChainSimulatorError> {
    println!("{}", "Attempting to start the Chain Simulator...".yellow());

    let mut child = Command::new(DOCKER_CMD)
        .args(["run", "-p", DEFAULT_PORT, SIMULATOR_IMAGE])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ChainSimulatorError::CommandFailed(format!(
            "Failed to execute `{DOCKER_CMD} run -p {DEFAULT_PORT} {SIMULATOR_IMAGE}`. Cause: {e:#?}"
        )))?;

    println!("{}", "Successfully started the Chain Simulator.".green());

    let stdout = child.stdout.take().ok_or_else(|| {
        ChainSimulatorError::OperationFailed("Failed to capture stdout.".to_string())
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        ChainSimulatorError::OperationFailed("Failed to capture stderr.".to_string())
    })?;

    let stdout_reader = io::BufReader::new(stdout);
    let stderr_reader = io::BufReader::new(stderr);

    for line in stdout_reader.lines().map_while(Result::ok) {
        println!("{line}");
    }

    let mut stderr_lines = Vec::new();
    for line in stderr_reader.lines().map_while(Result::ok) {
        eprintln!("{line}");
        stderr_lines.push(line);
    }

    let status = child.wait().map_err(|e| {
        ChainSimulatorError::OperationFailed(format!(
            "Waiting for the container process to finish failed. Cause: {e:#?}"
        ))
    })?;

    if status.success() {
        Ok(())
    } else {
        let stderr_msg = stderr_lines.join("\n");

        Err(ChainSimulatorError::OperationFailed(format!(
            "Chain Simulator execution failed. Exit status: {status}. Error stack trace: {stderr_msg}",
        )))
    }
}
