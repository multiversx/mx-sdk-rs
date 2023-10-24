use std::process::Command;

use crate::cli_args::TestArgs;

pub fn test(test_args: &TestArgs) {
    let path = test_args.path.as_deref().unwrap_or("./");
    let mut program = "cargo";
    let mut args = Vec::new();

    match test_args.test_type.as_str() {
        "scenario" => {
            program = "run-scenarios";
            args.extend(["./"]);
        },
        "go" => {
            args.extend(["test", "--features", "multiversx-sc-scenario/run-go-tests"]);
        },
        "rust" => {
            args.extend(["test"]);
        },
        _ => {
            panic!("Unrecognised test argument");
        },
    }

    let status = Command::new(program)
        .args(args.clone())
        .current_dir(path)
        .status()
        .expect(&format!("Failed to run program: {program} {:?}", args));

    println!("process finished with: {status}");
    assert!(status.success());
}
