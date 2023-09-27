mod git_describe;
pub mod twiggy;
mod wasm_imports;
mod wasm_opt;
mod wasm_to_wat;

pub use git_describe::git_describe;
pub use wasm_imports::extract_wasm_imports;
pub use wasm_opt::run_wasm_opt;
pub use wasm_to_wat::wasm_to_wat;

use crate::cli_args::BuildArgs;

pub fn check_tools_installed(build_args: &mut BuildArgs) {
    if build_args.wasm_opt && !wasm_opt::is_wasm_opt_installed() {
        println!("Warning: {} not installed", wasm_opt::WASM_OPT_NAME);
        build_args.wasm_opt = false;
    }
    if build_args.has_twiggy_call() && !twiggy::is_twiggy_installed() {
        println!("Warning: {} not installed", twiggy::TWIGGY_NAME);
        build_args.twiggy_top = false;
        build_args.twiggy_paths = false;
        build_args.twiggy_monos = false;
        build_args.twiggy_dominators = false;
    }
}
