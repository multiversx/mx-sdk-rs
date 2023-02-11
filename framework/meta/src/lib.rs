pub mod abi_json;
mod cargo_toml_contents;
pub mod cli_args;
mod folder_structure;
mod generate_snippets;
mod local_deps;
mod meta_abi;
mod meta_all;
mod meta_cli;
mod meta_config;
mod meta_info;
mod meta_validate_abi;
mod meta_wasm_tools;
pub mod output_contract;
mod sc_upgrade;

pub use cargo_toml_contents::CargoTomlContents;
pub use meta_cli::{cli_main, cli_main_standalone, multi_contract_config};

#[macro_use]
extern crate lazy_static;
