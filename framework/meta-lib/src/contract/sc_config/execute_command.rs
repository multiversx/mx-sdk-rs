use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::{Command, ExitStatus, Stdio};

use crate::print_util::format_command;

/// Contains a description of the command, for error reporting purposes.
#[derive(Clone, Debug)]
pub struct CommandInfo {
    command_string: String,
}

impl From<&mut Command> for CommandInfo {
    fn from(command: &mut Command) -> Self {
        CommandInfo {
            command_string: format_command(command),
        }
    }
}

impl Display for CommandInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command_string)
    }
}

/// Represents errors that can occur when executing system commands.
#[derive(Debug)]
pub enum ExecuteCommandError {
    ErrorRunning(CommandInfo),
    JobFailed(CommandInfo),
    ErrorParsing(CommandInfo),
    ErrorRunningBuildProcess,
}

impl Error for ExecuteCommandError {}

impl Display for ExecuteCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExecuteCommandError::ErrorRunning(command_info) => format!(
                "Error running `{command_info}`: ensure it is installed and available in your system PATH.",
            ),
            ExecuteCommandError::JobFailed(command_info) => {
                format!("Job failed, command: `{command_info}`")
            }
            ExecuteCommandError::ErrorParsing(command_info) => {
                format!("Error parsing output of `{command_info}`")
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
        .map_err(|_| ExecuteCommandError::ErrorRunning(command.into()))?;

    if !output.status.success() {
        return Err(ExecuteCommandError::JobFailed(command.into()));
    }

    String::from_utf8(output.stdout).map_err(|_| ExecuteCommandError::ErrorParsing(command.into()))
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
        return Err(ExecuteCommandError::JobFailed(command.into()));
    }

    Ok(response)
}
