use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::{Command, ExitStatus, Stdio};

use crate::print_util::format_command;

#[derive(Debug)]
pub enum ExecuteCommandError {
    ErrorRunning(String),
    JobFailed(String),
    ErrorParsing(String),
    ErrorRunningBuildProcess,
}

impl Error for ExecuteCommandError {}

impl Display for ExecuteCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExecuteCommandError::ErrorRunning(command_string) => format!(
                "Error running `{command_string}`: ensure it is installed and available in your system PATH.",
            ),
            ExecuteCommandError::JobFailed(command_string) => {
                format!("Job failed, command: `{command_string}`")
            }
            ExecuteCommandError::ErrorParsing(command_string) => {
                format!("Error parsing output of `{command_string}`")
            }
            ExecuteCommandError::ErrorRunningBuildProcess => {
                "contract build process was not running".to_string()
            }
        };
        write!(f, "{}", message)
    }
}

pub(crate) fn execute_command(command: &mut Command) -> Result<String, ExecuteCommandError> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| ExecuteCommandError::ErrorRunning(format_command(command)))?;

    if !output.status.success() {
        return Err(ExecuteCommandError::JobFailed(format_command(command)));
    }

    String::from_utf8(output.stdout)
        .map_err(|_| ExecuteCommandError::ErrorParsing(format_command(command)))
}

pub(crate) fn execute_spawn_command(
    command: &mut Command,
) -> Result<ExitStatus, ExecuteCommandError> {
    let response = command
        .spawn()
        .expect("failed to spawn contract build process")
        .wait()
        .map_err(|_| ExecuteCommandError::ErrorRunningBuildProcess)?;

    if !response.success() {
        return Err(ExecuteCommandError::JobFailed(format_command(command)));
    }

    Ok(response)
}
