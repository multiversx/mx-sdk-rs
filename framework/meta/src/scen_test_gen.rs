mod stg_main;
mod stg_model;
mod stg_parse;
mod stg_print;
mod stg_process_file;
mod stg_write;

use crate::cli_args::TestGenArgs;

pub fn test_gen_tool(args: &TestGenArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    stg_main::perform_test_gen_all(path, args.ignore.as_slice());
}
