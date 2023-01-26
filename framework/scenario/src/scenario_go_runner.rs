use std::{io::ErrorKind, path::Path, process::Command};

/// Runs the Arwen executable,
/// which reads parses and executes one or more mandos tests.
pub fn run_go<P: AsRef<Path>>(relative_path: P) {
    if cfg!(not(feature = "run-go-tests")) {
        return;
    }

    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);

    let run_scenarios_result = Command::new("run-scenarios")
        .arg(absolute_path.clone())
        .output();

    let not_found = if let Err(error) = &run_scenarios_result {
        error.kind() == ErrorKind::NotFound
    } else {
        false
    };

    let result = if not_found {
        // fallback - use the old binary
        println!("Warning: `run-scenarios` not found. Using `mandos-test` as fallback.");
        Command::new("mandos-test").arg(absolute_path).output()
    } else {
        run_scenarios_result
    };

    let output = result.expect("failed to execute process");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(output.stdout.as_slice()));
    } else {
        panic!(
            "Mandos-go output:\n{}\n{}",
            String::from_utf8_lossy(output.stdout.as_slice()),
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }
}
