use colored::Colorize;
use rustc_version::{Version, version_meta};
use std::{
    env,
    ffi::OsString,
    process::{Command, exit},
};

use crate::{
    contract::sc_config::execute_command::execute_command, print_util, tools::RustcVersion,
};

pub const WASM32_TARGET: &str = "wasm32-unknown-unknown";
pub const WASM32V1_TARGET: &str = "wasm32v1-none";
const FIRST_RUSTC_VERSION_WITH_WASM32V1_TARGET: Version = Version::new(1, 85, 0);

/// Gets the rustc wasm32 target name.
///
/// It is currently "wasm32v1-none", except for before Rust 1.85, where we use "wasm32-unknown-unknown".
pub fn default_target() -> &'static str {
    if is_wasm32v1_available() {
        WASM32V1_TARGET
    } else {
        WASM32_TARGET
    }
}

pub fn is_wasm32v1_available() -> bool {
    let Ok(version) = version_meta() else {
        return false;
    };

    version.semver >= FIRST_RUSTC_VERSION_WITH_WASM32V1_TARGET
}

fn rustup_command() -> Command {
    let rustup = env::var_os("RUSTUP").unwrap_or_else(|| OsString::from("rustup"));
    Command::new(rustup)
}

pub fn is_target_installed(rustc_version: &RustcVersion, target_name: &str) -> bool {
    let mut cmd = rustup_command();
    cmd.arg(rustc_version.to_cli_arg())
        .arg("target")
        .arg("list")
        .arg("--installed");

    print_util::print_rustup_check_target(rustc_version, target_name, &cmd);

    let output_rustup_command = execute_command(&mut cmd, "rustup");
    let str_output_rustup = match output_rustup_command {
        Ok(output) => output,
        Err(err) => {
            println!("\n{}", err.to_string().red().bold());
            exit(1);
        }
    };

    let installed = str_output_rustup.contains(target_name);

    print_util::print_rustup_check_target_result(installed);

    installed
}

pub fn install_target(rustc_version: Option<&RustcVersion>, target_name: &str) {
    let mut cmd = rustup_command();
    if let Some(rustc_version) = rustc_version {
        cmd.arg(rustc_version.to_cli_arg());
    }
    cmd.arg("target").arg("add").arg(target_name);

    print_util::print_rustup_install_target(target_name, &cmd);

    let exit_status = cmd.status().expect("failed to execute `rustup`");

    assert!(
        exit_status.success(),
        "failed to install {target_name} target"
    );

    print_util::print_rustup_install_target_success(target_name);
}
