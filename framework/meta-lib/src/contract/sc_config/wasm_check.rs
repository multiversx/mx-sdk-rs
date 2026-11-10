use std::process::Command;

use super::ContractVariant;
use crate::{cli::CheckArgs, tools::install_wasm_target};

impl ContractVariant {
    /// Runs `cargo check` in the corresponding wasm crate, for the wasm target.
    pub fn cargo_check(&self, check_args: &CheckArgs) {
        if !install_wasm_target::is_target_installed(
            &self.settings.rustc_version,
            &self.settings.rustc_target,
        ) {
            install_wasm_target::install_target(
                Some(&self.settings.rustc_version),
                &self.settings.rustc_target,
            );
        }

        let mut command = Command::new("cargo");
        command
            .arg(self.settings.rustc_version.to_cli_arg())
            .arg("check")
            .arg(format!("--target={}", self.settings.rustc_target))
            .arg("--release")
            .current_dir(self.wasm_crate_path());
        if check_args.locked {
            command.arg("--locked");
        }
        if check_args.frozen {
            command.arg("--frozen");
        }
        if check_args.offline {
            command.arg("--offline");
        }

        let exit_status = command
            .spawn()
            .expect("failed to spawn contract check process")
            .wait()
            .expect("contract check process was not running");

        assert!(exit_status.success(), "contract check process failed");
    }
}
