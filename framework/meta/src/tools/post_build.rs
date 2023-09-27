use std::process::{Command, Stdio};

use crate::cli_args::BuildArgs;

const WASM_OPT_NAME: &str = "wasm-opt";
const WASM2WAT_NAME: &str = "wasm2wat";
const TWIGGY_NAME: &str = "twiggy";

pub(crate) fn check_tools_installed(build_args: &mut BuildArgs) {
    if build_args.wasm_opt && !is_wasm_opt_installed() {
        println!("Warning: {WASM_OPT_NAME} not installed");
        build_args.wasm_opt = false;
    }
    if build_args.wat && !is_wasm2wat_installed() {
        println!("Warning: {WASM2WAT_NAME} not installed");
        build_args.wat = false;
    }
    if build_args.has_twiggy_call() && !is_twiggy_installed() {
        println!("Warning: {TWIGGY_NAME} not installed");
        build_args.twiggy_top = false;
        build_args.twiggy_paths = false;
        build_args.twiggy_monos = false;
        build_args.twiggy_dominators = false;
    }
}

fn is_wasm_opt_installed() -> bool {
    Command::new(WASM_OPT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn is_wasm2wat_installed() -> bool {
    Command::new(WASM2WAT_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn is_twiggy_installed() -> bool {
    Command::new(TWIGGY_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

pub(crate) fn run_wasm_opt(output_wasm_path: &str) {
    let exit_status = Command::new(WASM_OPT_NAME)
        .args([output_wasm_path, "-Oz", "--output", output_wasm_path])
        .spawn()
        .expect("failed to spawn wasm-opt process")
        .wait()
        .expect("wasm-opt was not running");

    assert!(exit_status.success(), "wasm-opt process failed");
}

pub(crate) fn run_wasm2wat(output_wasm_path: &str, output_wat_path: &str) {
    let exit_status = Command::new(WASM2WAT_NAME)
        .args([output_wasm_path, "--output", output_wat_path])
        .spawn()
        .expect("failed to spawn wasm2wat process")
        .wait()
        .expect("wasm2wat was not running");

    assert!(exit_status.success(), "wasm2wat process failed");
}

fn run_with_stdout_file<I, S>(stdout_file_name: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let stdout_file = std::fs::File::create(stdout_file_name).unwrap();
    let _ = Command::new(TWIGGY_NAME)
        .args(args)
        .stdout(Stdio::from(stdout_file))
        .spawn()
        .expect("failed to spawn twiggy process")
        .wait()
        .expect("twiggy was not running");
}

pub(crate) fn run_twiggy_top(output_wasm_path: &str, output_twiggy_top_path: &str) {
    run_with_stdout_file(
        output_twiggy_top_path,
        ["top", "-n", "1000", output_wasm_path],
    );
}

pub(crate) fn run_twiggy_paths(output_wasm_path: &str, output_twiggy_paths_path: &str) {
    run_with_stdout_file(output_twiggy_paths_path, ["paths", output_wasm_path]);
}

pub(crate) fn run_twiggy_monos(output_wasm_path: &str, output_twiggy_monos_path: &str) {
    run_with_stdout_file(output_twiggy_monos_path, ["monos", output_wasm_path]);
}

pub(crate) fn run_twiggy_dominators(output_wasm_path: &str, output_twiggy_dominators_path: &str) {
    run_with_stdout_file(
        output_twiggy_dominators_path,
        ["dominators", output_wasm_path],
    );
}
