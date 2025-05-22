use std::{
    env,
    ffi::OsString,
    path::Path,
    process::{Command, Stdio},
};

use colored::Colorize;
use multiversx_sc_meta_lib::contract::sc_config::ExecuteCommandError;

const WASMER_CRATE_NAME: &str = "multiversx-chain-vm-executor-wasmer ";
const WASMER_EXPERIMENTAL_CRATE_NAME: &str = "multiversx-chain-vm-executor-wasmer-experimental ";

pub fn check_wasmer_dependencies(path: &Path) {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"));

    let mut command = Command::new(cargo);
    command.arg("tree").arg("-e").arg("features");

    match execute_command(&mut command, path, "cargo") {
        Ok(output) => {
            if output.contains(WASMER_CRATE_NAME) && output.contains(WASMER_EXPERIMENTAL_CRATE_NAME)
            {
                println!(
                    "{}",
                    "WARNING: Importing both wasmer and wasmer-experimental will crash on some operating systems."
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
        return Err(ExecuteCommandError::JobFailed(job.to_string()));
    }

    String::from_utf8(output.stdout).map_err(|_| ExecuteCommandError::ErrorParsing(job.to_string()))
}
