pub use generate::{generate_file_content, generate_to_string};
use parse::parse_toml_sections;

mod generate;
mod parse;

pub const SECTIONS_FILE_PATH: &str = "../../chain/vm/src/schedule/gas_schedule_sections.rs";

pub fn generate() {
    generate_file_content(&std::path::PathBuf::from(SECTIONS_FILE_PATH));
}
