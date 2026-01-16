use clap::Parser;

pub use generate::generate_file_content;
use parse::parse_toml_sections;
use util::get_file_path;

mod cli;
mod generate;
mod parse;
mod util;

pub fn generate() {
    env_logger::init();

    let cli = cli::Cli::parse();
    match &cli.command {
        Some(cli::CliCommand::Generate(arg)) => {
            generate_file_content(arg.toml_version);
        }
        None => {}
    }
}
