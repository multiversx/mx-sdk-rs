use std::process::Command;

use colored::Colorize;

use crate::cli::TestInteractorsArgs;

pub fn test_interactors(args: &TestInteractorsArgs) {
    let path = args.path.as_deref().unwrap_or("./");
    let command = "cargo";
    let mut command_args = Vec::new();

    let no_capture = args.nocapture;

    command_args.push("test");

    command_args.extend(["--features", "chain_simulator"]);

    if no_capture {
        command_args.extend(["--", "--nocapture"]);
    }

    let args_str = command_args.join(" ");

    println!(
        "{}\n{}",
        format!("Running tests in {path} ...").green(),
        format!("Executing {command} {args_str} ...").green()
    );

    let status = Command::new(command)
        .args(command_args.clone())
        .current_dir(path)
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                format!("Failed to run program: {command} {args_str}").bright_red()
            )
        });

    println!("Process finished with: {status}");
    assert!(status.success());
}
