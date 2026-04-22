use std::{path::Path, process::Command};

use multiversx_sc_meta_lib::cli::{CliArgsToRaw, ContractCliAction};

use crate::{
    cli::MetaLibArgs, cmd::print_util::print_all_command, folder_structure::RelevantDirectory,
};

/// Formatted cargo arguments for calling a contract's meta crate.
///
/// Wraps the `Vec<String>` produced by combining a [`ContractCliAction`] with
/// [`MetaLibArgs`], applying the `--target-dir-all` overrides before serialising.
pub struct ContractMetaCall(Vec<String>);

impl ContractMetaCall {
    pub fn new(mut command: ContractCliAction, meta_lib_args: &MetaLibArgs) -> Self {
        apply_target_dir_all_to_wasm(&mut command, meta_lib_args);

        let mut raw = Vec::new();
        if let Some(target_dir_meta) = effective_target_dir_meta(meta_lib_args) {
            raw.push("--target-dir".to_string());
            raw.push(target_dir_meta.clone());
        }
        raw.append(&mut command.to_raw());
        if !meta_lib_args.load_abi_git_version {
            raw.push("--no-abi-git-version".to_string());
        }

        ContractMetaCall(raw)
    }

    /// Creates a `ContractMetaCallArgs` directly from a raw argument list,
    /// bypassing the standard construction logic.
    pub fn from_raw(args: Vec<String>) -> Self {
        ContractMetaCall(args)
    }

    pub fn args(&self) -> &[String] {
        &self.0
    }

    pub fn call_in_dir(&self, meta_path: &Path) {
        print_all_command(meta_path, self.args());

        let exit_status = Command::new("cargo")
            .current_dir(meta_path)
            .arg("run")
            .arg("--")
            .args(self.args())
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
