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
            println!("{}", format!("If scen parameter is true, it will override the go parameter. Executing scenarios...").yellow());
        }
    } else if go {
        args.extend(["test", "--features", "multiversx-sc-scenario/run-go-tests"]);
    } else {
        args.extend(["test"]);
    }

    println!("{}", format!("Executing {program} {:?} ...", args).green());

    let status = Command::new(program)
        .args(args.clone())
        .current_dir(path)
        .status()
        .expect(&format!("Failed to run program: {program} {:?}", args).bright_red());

    println!("Process finished with: {status}");
    assert!(status.success());
}
