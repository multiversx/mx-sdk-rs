use colored::Colorize;
use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::{Command, Output},
};

const RUNNER_TOOL_NAME: &str = "mx-scenario-go";
const RUNNER_TOOL_NAME_LEGACY: &str = "run-scenarios";

/// Just marks that the tool was not found.
struct ToolNotFound;

/// Runs the VM executable,
/// which reads parses and executes one or more mandos tests.
pub fn run_mx_scenario_go(absolute_path: &Path) {
    if cfg!(not(feature = "run-go-tests")) {
        return;
    }

    let output = Command::new(RUNNER_TOOL_NAME)
        .arg("run")
        .arg(absolute_path)
        .output();
    if run_scenario_tool(RUNNER_TOOL_NAME, output).is_ok() {
        return;
    }

    // fallback - use the old binary
    println!(
        "{}",
        format!("Warning: `{RUNNER_TOOL_NAME}` not found. Using `{RUNNER_TOOL_NAME_LEGACY}` as fallback.").yellow(),
    );
    let output = Command::new(RUNNER_TOOL_NAME_LEGACY)
        .arg(absolute_path)
        .output();
    if run_scenario_tool(RUNNER_TOOL_NAME_LEGACY, output).is_ok() {
        return;
    }

    panic!("Could not find `{RUNNER_TOOL_NAME_LEGACY}`, aborting.");
}

fn run_scenario_tool(tool_name: &str, output: Result<Output, Error>) -> Result<(), ToolNotFound> {
    if let Err(error) = &output {
        if error.kind() == ErrorKind::NotFound {
            return Err(ToolNotFound);
        }
    }

    let output = output.expect("failed to execute process");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
    } else {
        panic!(
            "{} output:\n{}\n{}",
            tool_name,
            String::from_utf8_lossy(output.stdout.as_slice()),
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }

    Ok(())
}
