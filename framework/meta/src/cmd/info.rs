use std::path::Path;

use crate::{
    cli::InfoArgs,
    folder_structure::{RelevantDirectories, dir_pretty_print},
    version_history::LAST_UPGRADE_VERSION,
};

use super::{
    check_wasmer_dependencies::check_wasmer_dependencies, print_util::print_tree_dir_metadata,
};

pub fn call_info(args: &InfoArgs) {
    let path = if let Some(some_path) = &args.path {
        Path::new(some_path)
    } else {
        Path::new("./")
    };

    check_wasmer_dependencies(path);

    let dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, &LAST_UPGRADE_VERSION)
    });
}
