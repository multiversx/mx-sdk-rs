use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::process::{Command, ExitStatus, Stdio};

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
            ExecuteCommandError::ErrorRunning(job) => format!(
                "Error running {}: ensure it is installed and available in your system PATH.",
                job
            ),
            ExecuteCommandError::JobFailed(job) => {
                format!("Job {} failed.", job)
            },
            ExecuteCommandError::ErrorParsing(job) => format!("Error parsing {} output", job),
            ExecuteCommandError::ErrorRunningBuildProcess => {
                "contract build process was not running".to_string()
            },
        };
        write!(f, "{}", message)
    }
}

pub(crate) fn execute_command(
    command: &mut Command,
    job: &str,
) -> Result<String, ExecuteCommandError> {
    let output = command
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| ExecuteCommandError::ErrorRunning(job.to_string()))?;

    if !output.status.success() {
        return Err(ExecuteCommandError::JobFailed(job.to_string()));
    }

    String::from_utf8(output.stdout).map_err(|_| ExecuteCommandError::ErrorParsing(job.to_string()))
}

pub(crate) fn execute_spawn_command(
    command: &mut Command,
    job: &str,
) -> Result<ExitStatus, ExecuteCommandError> {
    let response = command
        .spawn()
        .expect("failed to spawn contract build process")
        .wait()
        .map_err(|_| ExecuteCommandError::ErrorRunningBuildProcess)?;

    if !response.success() {
        return Err(ExecuteCommandError::JobFailed(job.to_string()));
    }

    Ok(response)
}
