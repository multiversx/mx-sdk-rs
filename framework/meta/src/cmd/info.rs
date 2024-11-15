use crate::{
    cli::InfoArgs,
    folder_structure::{dir_pretty_print, RelevantDirectories},
    version_history::LAST_UPGRADE_VERSION,
};

use super::print_util::print_tree_dir_metadata;

pub fn call_info(args: &InfoArgs) {
    let path = if let Some(some_path) = &args.path {
        some_path.as_str()
    } else {
        "./"
    };

    let dirs = RelevantDirectories::find_all(path, args.ignore.as_slice());
    dir_pretty_print(dirs.iter(), "", &|dir| {
        print_tree_dir_metadata(dir, &LAST_UPGRADE_VERSION)
    });
}
