// TODO: remove once minimum version is 1.87+
#![allow(unknown_lints)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_is_multiple_of)]

pub mod abi_json;
pub mod cargo_toml;
pub mod cli;
pub mod code_report_json;
pub mod contract;
pub mod ei;
pub mod ei_check_json;
pub mod esdt_attr_file_json;
pub mod mxsc_file_json;
pub mod print_util;
pub mod report_info_json;
pub mod tools;
pub mod version;
pub mod version_history;

#[macro_use]
extern crate lazy_static;

pub use cli::{cli_main, multi_contract_config};
