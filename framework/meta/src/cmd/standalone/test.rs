use std::process::Command;

use colored::Colorize;

use crate::cli_args::TestArgs;

pub fn test(test_args: &TestArgs) {
    let path = test_args.path.as_deref().unwrap_or("./");
    let mut program = "cargo";
    let mut args = Vec::new();

    let go = test_args.go;
    let scen = test_args.scen;

    if scen {
        program = "run-scenarios";
        args.extend(["./"]);

        if go {
            println!("{}", "If scen parameter is true, it will override the go parameter. Executing scenarios...".yellow());
        }
    } else if go {
        args.extend(["test", "--features", "multiversx-sc-scenario/run-go-tests"]);
    } else {
        args.extend(["test"]);
    }

    let args_str = args.join(" ");

    println!(
        "{}\n{}",
        format!("Running tests in {path} ...").green(),
        format!("Executing {program} {args_str} ...").green()
    );

    let status = Command::new(program)
        .args(args.clone())
        .current_dir(path)
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                format!("Failed to run program: {program} {args_str}").bright_red()
            )
        });

    println!("Process finished with: {status}");
    assert!(status.success());
}
