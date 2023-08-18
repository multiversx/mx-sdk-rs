mod all;
mod info;
mod local_deps;
pub mod scen_test_gen;
pub(crate) mod upgrade;

use crate::{
    cli_args::{StandaloneCliAction, StandaloneCliArgs},
    template::{create_contract, print_template_names},
};
use all::call_all_meta;
use clap::Parser;
use info::call_info;
use local_deps::local_deps;
use scen_test_gen::test_gen_tool;
use upgrade::upgrade_sc;

/// Entry point in the program when calling it as a standalone tool.
pub async fn cli_main_standalone() {
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
            create_contract(args).await;
        },
        Some(StandaloneCliAction::TemplateList(args)) => {
            print_template_names(args).await;
        },
        Some(StandaloneCliAction::TestGen(args)) => {
            test_gen_tool(args);
        },
        None => {},
    }
}
