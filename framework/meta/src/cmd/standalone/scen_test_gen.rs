mod stg_main;
mod stg_parse;
mod stg_print;
mod stg_process_code;
mod stg_section;
mod stg_write;

use crate::cli_args::TestGenArgs;

pub fn test_gen_tool(args: &TestGenArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    stg_main::perform_test_gen_all(path, args.ignore.as_slice(), args.create);
}

// Good for testing.
pub use stg_process_code::process_code;
pub use stg_write::{
    format_test_fn_go, format_test_fn_rs, WriteTestFn, DEFAULT_SETUP_GO, DEFAULT_SETUP_RS,
};
