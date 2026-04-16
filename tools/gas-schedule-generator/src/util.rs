use std::path::PathBuf;

const SECTIONS_FILE: &str = "../../chain/vm/src/schedule/gas_schedule_sections.rs";

pub(crate) fn get_file_path() -> PathBuf {
    PathBuf::from(SECTIONS_FILE)
}
