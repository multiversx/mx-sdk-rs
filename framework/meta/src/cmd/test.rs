use std::process::Command;

use colored::Colorize;

use crate::cli::TestArgs;

pub fn test(test_args: &TestArgs) {
    let path = test_args.path.as_deref().unwrap_or("./");
    let mut program = "cargo";
    let mut args = Vec::new();

    if test_args.scen {
        program = "mx-scenario-go";
        args.extend(["run", "./"]);

        if test_args.go {
            println!("{}", "If scen parameter is true, it will override the go parameter. Executing scenarios...".yellow());
        }
    } else {
        args.push("test");

        if test_args.go {
            args.extend(["--features", "multiversx-sc-scenario/run-go-tests"]);
        }

        if test_args.wasm {
            args.extend(["--features", "multiversx-sc-scenario/compiled-sc-tests"]);
        }

        if test_args.chain_simulator {
            args.extend(["--features", "chain-simulator-tests"]);
        }

        if test_args.nocapture {
            args.extend(["--", "--nocapture"]);
        }
    }

    let args_str = args.join(" ");

    println!(
        "{}\n{}",
        format!("Running tests in {path} ...").green(),
        format!("Executing {program} {args_str} ...").green()
    );

    let status = Command::new(program)
        .args(args)
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
