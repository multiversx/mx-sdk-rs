use colored::Colorize;
use std::{io::ErrorKind, path::Path, process::Command};

const RUNNER_TOOL_NAME: &str = "run-scenarios";
const RUNNER_TOOL_NAME_LEGACY: &str = "mandos-test";

/// Just marks that the tool was not found.
struct ToolNotFound;

/// Runs the VM executable,
/// which reads parses and executes one or more mandos tests.
pub fn run_vm_go_tool(absolute_path: &Path) {
    if cfg!(not(feature = "run-go-tests")) {
        return;
    }

    if run_scenario_tool(RUNNER_TOOL_NAME, absolute_path).is_ok() {
        return;
    }

    // fallback - use the old binary
    println!(
        "{}",
        format!("Warning: `{RUNNER_TOOL_NAME}` not found. Using `{RUNNER_TOOL_NAME_LEGACY}` as fallback.").yellow(),
    );
    if run_scenario_tool(RUNNER_TOOL_NAME_LEGACY, absolute_path).is_ok() {
        return;
    }

    panic!("Could not find `{RUNNER_TOOL_NAME_LEGACY}`, aborting.");
}

fn run_scenario_tool(tool_name: &str, path: &Path) -> Result<(), ToolNotFound> {
    let result = Command::new(tool_name).arg(path).output();

    if let Err(error) = &result {
        if error.kind() == ErrorKind::NotFound {
            return Err(ToolNotFound);
        }
    }

    let output = result.expect("failed to execute process");

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
