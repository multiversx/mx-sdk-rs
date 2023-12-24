mod all;
mod info;
mod local_deps;
mod print_util;
pub mod scen_test_gen;
pub mod template;
pub mod test;
pub(crate) mod upgrade;

use crate::cli_args::{StandaloneCliAction, StandaloneCliArgs};
use all::call_all_meta;
use clap::Parser;
use info::call_info;
use local_deps::local_deps;
use scen_test_gen::test_gen_tool;
use template::{create_contract, print_template_names};
use test::test;
use upgrade::upgrade_sc;

/// Entry point in the program when calling it as a standalone tool.
pub fn cli_main_standalone() {
    let cli_args = StandaloneCliArgs::parse();
    match &cli_args.command {
        Some(StandaloneCliAction::Info(args)) => call_info(args),
        Some(StandaloneCliAction::All(args)) => call_all_meta(args),
        Some(StandaloneCliAction::Upgrade(args)) => {
            upgrade_sc(args);
        },
        Some(StandaloneCliAction::LocalDeps(args)) => {
            local_deps(args);
        },
        Some(StandaloneCliAction::Template(args)) => {
            create_contract(args);
        },
        Some(StandaloneCliAction::TemplateList(args)) => {
            print_template_names(args);
        },
        Some(StandaloneCliAction::TestGen(args)) => {
            test_gen_tool(args);
        },
        Some(StandaloneCliAction::Test(args)) => test(args),
        None => {},
    }
}
