use std::process::Command;

use colored::Colorize;
use interactor_tests::perform_tests_interactor;

use crate::cli::TestArgs;

mod interactor_test_analyzer;
mod interactor_tests;
mod simulator_setup;

pub fn test(test_args: &TestArgs) {
    let path = test_args.path.as_deref().unwrap_or("./");
    let mut program = "cargo";
    let mut args = Vec::new();

    let go = test_args.go;
    let scen = test_args.scen;
    let no_capture = test_args.nocapture;
    let chain_simulator = test_args.chain_simulator;

    if chain_simulator && !go {
        perform_tests_interactor(path);
        return;
    }

    if scen {
        program = "mx-scenario-go";
        args.extend(["run", "./"]);

        if go {
            println!("{}", "If scen parameter is true, it will override the go parameter. Executing scenarios...".yellow());
        }

        execute_and_print(path, program, &args);
    }

    args.push("test");

    if go {
        args.extend(["--features", "multiversx-sc-scenario/run-go-tests"]);
    }

    if no_capture {
        args.extend(["--", "--nocapture"]);
    }

    if chain_simulator {
        perform_tests_interactor(path);
    }

    execute_and_print(path, program, &args);
}

fn execute_and_print(file_path: &str, program: &str, args: &[&str]) {
    let args_str = args.join(" ");

    println!(
        "{}\n{}",
        format!("Running tests in {file_path} ...").green(),
        format!("Executing {program} {args_str} ...").green()
    );

    let status = Command::new(program)
        .args(args)
        .current_dir(file_path)
        .status()
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                format!("Failed to run: {program} {args_str}").bright_red()
            )
        });
    println!("Process finished with: {status}");
    assert!(status.success());
}
