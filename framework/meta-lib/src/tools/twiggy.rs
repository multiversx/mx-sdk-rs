use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const TWIGGY_NAME: &str = "twiggy";

pub fn is_twiggy_installed() -> bool {
    Command::new(TWIGGY_NAME)
        .args(["--version"])
        .output()
        .is_ok()
}

fn run_with_stdout_file<I, S>(stdout_file_name: &PathBuf, args: I)
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

pub(crate) fn run_twiggy_top(output_wasm_path: &Path, output_twiggy_top_path: &PathBuf) {
    run_with_stdout_file(
        output_twiggy_top_path,
        ["top", "-n", "1000", &output_wasm_path.to_string_lossy()],
    );
}

pub(crate) fn run_twiggy_paths(output_wasm_path: &Path, output_twiggy_paths_path: &PathBuf) {
    run_with_stdout_file(
        output_twiggy_paths_path,
        ["paths", &output_wasm_path.to_string_lossy()],
    );
}

pub(crate) fn run_twiggy_monos(output_wasm_path: &Path, output_twiggy_monos_path: &PathBuf) {
    run_with_stdout_file(
        output_twiggy_monos_path,
        ["monos", &output_wasm_path.to_string_lossy()],
    );
}

pub(crate) fn run_twiggy_dominators(
    output_wasm_path: &Path,
    output_twiggy_dominators_path: &PathBuf,
) {
    run_with_stdout_file(
        output_twiggy_dominators_path,
        ["dominators", &output_wasm_path.to_string_lossy()],
    );
}
