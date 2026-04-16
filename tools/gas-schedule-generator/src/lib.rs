pub use generate::{generate_file_content, generate_to_string};
use parse::parse_toml_sections;
use util::get_file_path;

mod generate;
mod parse;
mod util;

pub fn generate() {
    env_logger::init();
    generate_file_content();
}
