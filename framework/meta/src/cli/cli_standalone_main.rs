use crate::cli::{StandaloneCliAction, StandaloneCliArgs};
use crate::cmd::chain_simulator::chain_simulator;
use crate::cmd::retrieve_address::retrieve_address;
use crate::cmd::wallet::wallet;
use clap::Parser;

use crate::cmd::all::call_all_meta;
use crate::cmd::code_report::report;
use crate::cmd::info::call_info;
use crate::cmd::install::install;
use crate::cmd::local_deps::local_deps;
use crate::cmd::scen_test_gen::test_gen_tool;
use crate::cmd::template::{create_contract, print_template_names};
use crate::cmd::test::test;
use crate::cmd::test_coverage::test_coverage;

use crate::cmd::upgrade::upgrade_sc;

/// Entry point in the program when calling it as a standalone tool.
pub async fn cli_main_standalone() {
    let cli_args = StandaloneCliArgs::parse();
    match &cli_args.command {
        Some(StandaloneCliAction::Info(args)) => call_info(args),
        Some(StandaloneCliAction::Install(args)) => install(args).await,
        Some(StandaloneCliAction::All(args)) => call_all_meta(args),
        Some(StandaloneCliAction::Upgrade(args)) => {
            upgrade_sc(args);
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
        Some(StandaloneCliAction::Test(args)) => test(args),
        Some(StandaloneCliAction::TestCoverage(args)) => {
            test_coverage(args);
        },
        Some(StandaloneCliAction::CodeReportGen(args)) => {
            report(args);
        },
        Some(StandaloneCliAction::Account(args)) => {
            retrieve_address(args).await;
        },
        Some(StandaloneCliAction::LocalDeps(args)) => {
            local_deps(args);
        },
        Some(StandaloneCliAction::Wallet(args)) => {
            wallet(args);
        },
        Some(StandaloneCliAction::ChainSimulator(args)) => {
            chain_simulator(args);
        },
        None => {},
    }
}
