use colored::*;
use std::fmt;

pub const DOCKER_CMD: &str = "docker";
pub const SIMULATOR_IMAGE: &str = "multiversx/chainsimulator:latest";
pub const DEFAULT_PORT: &str = "8085:8085";

#[derive(Debug)]
pub enum ChainSimulatorError {
    DockerNotInstalled,
    CommandFailed(String),
    OperationFailed(String),
    ContainerNotRunning,
}

impl fmt::Display for ChainSimulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainSimulatorError::DockerNotInstalled => {
                write!(
                    f,
                    "{}",
                    "Error: Docker is not installed. Please install Docker to continue.".red()
                )
            }
            ChainSimulatorError::CommandFailed(cmd) => {
                write!(f, "{} {}", "Error: Failed to execute command:".red(), cmd)
            }
            ChainSimulatorError::OperationFailed(op) => {
                write!(f, "{} {}", "Error: Operation failed:".red(), op)
            }
            ChainSimulatorError::ContainerNotRunning => {
                write!(
                    f,
                    "{}",
                    "Warning: No running Chain Simulator container found.".yellow()
                )
            }
        }
    }
}

impl std::error::Error for ChainSimulatorError {}
