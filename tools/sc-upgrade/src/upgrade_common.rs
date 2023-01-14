
use std::path::Path;

use ruplacer::{Query, Console, Settings, DirectoryPatcher};

pub(crate) fn replace_in_files(sc_crate_path: &Path, file_type: &str, queries: &[Query]) {
    let console = Console::default();
    let settings = Settings {
        selected_file_types: vec![file_type.to_string()],
        ..Default::default()
    };
    let mut directory_patcher = DirectoryPatcher::new(&console, &sc_crate_path, &settings);
    for query in queries {
        directory_patcher.run(&query).expect("replace failed");
    }
}