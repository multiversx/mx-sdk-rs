use std::{
    env,
    error::Error,
    ffi::OsString,
    fmt::{Display, Formatter},
    path::Path,
    process::{Command, ExitStatus, Stdio},
};

use colored::Colorize;

const WASMER_PRODUCT: &str = "multiversx-chain-vm-executor-wasmer ";
const WASMER_EXPERIMENTAL: &str = "multiversx-chain-vm-executor-wasmer-experimental ";

#[derive(Debug)]
pub enum ExecuteCommandError {
    ErrorRunning(String),
    JobFailed(String, ExitStatus),
    ErrorParsing(String),
}

impl Error for ExecuteCommandError {}

impl Display for ExecuteCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ExecuteCommandError::ErrorRunning(job) => format!("Error running {}", job),
            ExecuteCommandError::JobFailed(job, status) => {
                format!("Command {} returned {}", job, status)
            },
            ExecuteCommandError::ErrorParsing(job) => format!("Error parsing {} output", job),
        };
        write!(f, "ExecuteCommandError: {}", message)
    }
}

pub fn check_wasmer_executor(path: &Path) {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    let mut command = Command::new(cargo);
    command.arg("tree").arg("-e").arg("features");

    match execute_command(&mut command, path, "cargo") {
        Ok(output) => {
            if output.contains(WASMER_PRODUCT) && output.contains(WASMER_EXPERIMENTAL) {
                println!(
                    "{}",
                    "Cannot import two different executors: found multiple wasmer components."
                        .to_string()
                        .red()
                        .bold(),
                );
            }
        },
        Err(err) => {
            println!("{}", err.to_string().to_string().red().bold());
        },
    };
}

fn execute_command(
    command: &mut Command,
    path: &Path,
    job: &str,
) -> Result<String, ExecuteCommandError> {
    let output = command
        .current_dir(path)
        .stderr(Stdio::inherit())
        .output()
        .map_err(|_| ExecuteCommandError::ErrorRunning(job.to_string()))?;

    if !output.status.success() {
        return Err(ExecuteCommandError::JobFailed(
            job.to_string(),
            output.status,
        ));
    }

    String::from_utf8(output.stdout).map_err(|_| ExecuteCommandError::ErrorParsing(job.to_string()))
}
