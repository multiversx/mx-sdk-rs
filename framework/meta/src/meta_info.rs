use crate::{
    cli_args::InfoArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories},
    sc_upgrade::{print_tree_dir_metadata, DEFAULT_LAST_VERSION},
};

pub fn call_info(args: &InfoArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, DEFAULT_LAST_VERSION)
    });
}
