use std::path::{Path, PathBuf};

const SECTIONS_FILE_NAME: &str = "sections.rs";
const OUTPUT_FILE_PATH_NAME: &str = "output_file_path.txt";

pub(crate) fn get_file_path() -> PathBuf {
    match std::fs::read_to_string(OUTPUT_FILE_PATH_NAME) {
        Ok(output_file_path) => PathBuf::from(output_file_path.trim()),
        Err(_) => {
            let output_dir = Path::new("output");
            std::fs::create_dir_all(output_dir).unwrap();

            output_dir.join(SECTIONS_FILE_NAME)
        }
    }
}
