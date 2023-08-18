pub mod abi_json;
mod cargo_toml_contents;
pub mod cli_args;
pub mod cmd;
pub mod ei;
mod folder_structure;
mod mxsc_file_json;
mod print_util;
pub mod template;
mod tools;
pub mod version_history;

#[macro_use]
extern crate lazy_static;

pub use cargo_toml_contents::CargoTomlContents;
pub use cmd::{
    contract::{cli_main, multi_contract_config},
    standalone::cli_main_standalone,
};
