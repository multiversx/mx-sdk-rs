use std::{
    env, path,
    process::{Command, ExitStatus},
};

use crate::cli_args::TestArgs;

// fn run_cargo_test(features: Option<&str>) -> ExitStatus {
//     let mut cargo_test = Command::new("cargo");
//     cargo_test.arg("test");

//     if let Some(features) = features {
//         cargo_test.arg("--features").arg(features);
//     }

//     cargo_test.status().expect("Failed to run 'cargo test'")
// }

pub fn test(args: &TestArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let status = Command::new("cargo")
        .args(&["test", "--features", "multiversx-sc-scenario/run-go-tests"])
        .current_dir(path)
        .status()
        .expect("Failed to run 'cargo test'");    


    println!("process finished with: {status}");

    // assert!(status.success());

    // let mut cargo_test = Command::new("cargo");
    // cargo_test.arg("test");
    // cargo_test.arg("--features").arg("multiversx-sc-scenario/run-go-tests");

    // cargo_test.current_dir(&path);

    // cargo_test.status().expect("");
}

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 || args[1] != "test" {
//         eprintln!("Invalid or missing subcommand. Usage: sc-meta test [--go]");
//         std::process::exit(1);
//     }

//     if args.len() > 2 && args[2] == "--go" {
//         run_cargo_test(Some("multiversx-sc-scenario/run-go-tests")); //wasm file exists
//     } else {
//         run_cargo_test(None);
//     }
// }
