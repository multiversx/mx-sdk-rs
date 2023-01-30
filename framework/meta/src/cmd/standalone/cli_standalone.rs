use clap::Parser;

use crate::cli_args::{StandaloneCliAction, StandaloneCliArgs};

use super::{all::call_all_meta, info::call_info, upgrade::upgrade_sc};

/// Entry point in the program when calling it as a standalone tool.
pub fn cli_main_standalone() {
    let cli_args = StandaloneCliArgs::parse();
    match &cli_args.command {
        Some(StandaloneCliAction::Info(args)) => call_info(args),
        Some(StandaloneCliAction::All(args)) => call_all_meta(args),
        Some(StandaloneCliAction::Upgrade(args)) => {
            upgrade_sc(args);
        },
        None => {},
    }
}
