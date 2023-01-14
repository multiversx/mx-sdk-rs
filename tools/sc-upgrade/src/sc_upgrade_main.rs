// mod cargo_toml_contents;

// use ruplacer::{Query, Console};

use clap::{command, Parser};
use multiversx_sc_upgrade::upgrade_sc;

#[derive(Debug, Parser)]
#[command(version, after_help = "MultiversX smart contract upgrade tool")]
struct Options {
    #[arg(help = "Explicitly provide the path to operate on")]
    path: Option<String>,
}

fn main() {
    let options = Options::parse();

    let path = if let Some(some_path) = &options.path {
        some_path.as_str()
    } else {
        ""
    };

    upgrade_sc(path);
}
