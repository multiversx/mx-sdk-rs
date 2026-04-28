use std::{path::Path, process::Command};

use multiversx_sc_meta_lib::cli::{CliArgsToRaw, ContractCliAction};

use crate::{
    cli::MetaLibArgs, cmd::print_util::print_all_command, folder_structure::RelevantDirectory,
};

/// Holds the arguments for calling a contract's meta crate via `cargo run`.
///
/// Separates cargo-level args (e.g. `--target-dir`, placed before `--`)
/// from binary-level args (the subcommand and its flags, placed after `--`).
pub struct ContractMetaCall {
    /// Args passed to `cargo run` itself (e.g. `--target-dir <dir>`).
    cargo_args: Vec<String>,
    /// Args passed to the compiled meta binary (e.g. `build --locked`).
    binary_args: Vec<String>,
}

impl ContractMetaCall {
    pub fn new(mut command: ContractCliAction, meta_lib_args: &MetaLibArgs) -> Self {
        apply_target_dir_all_to_wasm(&mut command, meta_lib_args);

        let mut cargo_args = Vec::new();
        if let Some(target_dir_meta) = effective_target_dir_meta(meta_lib_args) {
            cargo_args.push("--target-dir".to_string());
            cargo_args.push(target_dir_meta.clone());
        }

        let mut binary_args = command.to_raw();
        if !meta_lib_args.load_abi_git_version {
            binary_args.push("--no-abi-git-version".to_string());
        }

        ContractMetaCall {
            cargo_args,
            binary_args,
        }
    }

    /// Creates a `ContractMetaCall` from a raw binary argument list,
    /// with no cargo-level args.
    pub fn from_raw(binary_args: Vec<String>) -> Self {
        ContractMetaCall {
            cargo_args: Vec::new(),
            binary_args,
        }
    }

    pub fn binary_args(&self) -> &[String] {
        &self.binary_args
    }

    /// Returns the full argument list for `cargo`, starting with `run`.
    fn all_cargo_args(&self) -> Vec<String> {
        let mut all = vec!["run".to_string()];
        all.extend_from_slice(&self.cargo_args);
        all.push("--".to_string());
        all.extend_from_slice(&self.binary_args);
        all
    }

    pub fn call_in_dir(&self, meta_path: &Path) {
        let all = self.all_cargo_args();
        print_all_command(meta_path, &all);

        let exit_status = Command::new("cargo")
            .current_dir(meta_path)
            .args(&all)
            .spawn()
            .expect("failed to spawn cargo run process in meta crate")
            .wait()
            .expect("cargo run process in meta crate was not running");

        assert!(exit_status.success(), "contract meta process failed");
    }

    pub fn call_for_contract(&self, contract_crate: &RelevantDirectory) {
        let meta_path = contract_crate.meta_path();
        self.call_in_dir(&meta_path);
    }
}

/// `--target-dir-all` overrides `--target-dir-meta`; returns whichever is set.
fn effective_target_dir_meta(meta_lib_args: &MetaLibArgs) -> Option<&String> {
    meta_lib_args
        .target_dir_all
        .as_ref()
        .or(meta_lib_args.target_dir_meta.as_ref())
}

/// Applies the `--target-dir-all` override to the wasm target dir inside the command.
fn apply_target_dir_all_to_wasm(command: &mut ContractCliAction, meta_lib_args: &MetaLibArgs) {
    if let Some(target_dir_all) = &meta_lib_args.target_dir_all {
        match command {
            ContractCliAction::Build(build_args) => {
                build_args.target_dir_wasm = Some(target_dir_all.clone());
            }
            ContractCliAction::BuildDbg(build_args) => {
                build_args.target_dir_wasm = Some(target_dir_all.clone());
            }
            ContractCliAction::Twiggy(build_args) => {
                build_args.target_dir_wasm = Some(target_dir_all.clone());
            }
            _ => {}
        }
    }
}
