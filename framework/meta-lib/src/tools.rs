pub mod build_target;
mod find_workspace;
mod git_describe;
pub(crate) mod panic_report;
mod rustc_version;
mod rustc_version_warning;
pub mod twiggy;
mod wasm_extractor;
pub mod wasm_opt;
mod wasm_to_wat;

pub use find_workspace::{find_current_workspace, find_workspace};
pub use git_describe::git_describe;
pub use rustc_version::RustcVersion;
pub use rustc_version_warning::rustc_version_warning;
pub use wasm_extractor::OpcodeVersion;
pub use wasm_extractor::code_report::CodeReport;
pub use wasm_extractor::extractor::WasmInfo;
pub use wasm_extractor::report::WasmReport;
pub use wasm_opt::install_wasm_opt;
pub use wasm_opt::run_wasm_opt;
pub use wasm_to_wat::wasm_to_wat;

use crate::cli::BuildArgs;

pub fn check_tools_installed(build_args: &mut BuildArgs) {
    if build_args.has_twiggy_call() && !twiggy::is_twiggy_installed() {
        println!("Warning: {} not installed", twiggy::TWIGGY_NAME);
        build_args.twiggy_top = false;
        build_args.twiggy_paths = false;
        build_args.twiggy_monos = false;
        build_args.twiggy_dominators = false;
    }
}
