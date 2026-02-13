use std::path::Path;

use crate::{
    cli::{ContractCliAction, ContractCliArgs},
    contract::{meta_config::MetaConfig, sc_config::ScConfig, scen_blackbox},
};
use clap::Parser;
use multiversx_sc::contract_base::ContractAbiProvider;

/// Entry point in the program from the contract meta crates.
pub fn cli_main<AbiObj: ContractAbiProvider>() {
    let cli_args = ContractCliArgs::parse();
    let mut meta_config_opt = process_original_abi::<AbiObj>(&cli_args);
    match cli_args.command {
        ContractCliAction::Abi => {}
        ContractCliAction::Build(build_args) => meta_config_opt.build(build_args),
        ContractCliAction::BuildDbg(build_args) => {
            meta_config_opt.build(build_args.into_build_args())
        }
        ContractCliAction::Twiggy(build_args) => {
            meta_config_opt.build(build_args.into_build_args())
        }
        ContractCliAction::Clean => meta_config_opt.clean(),
        ContractCliAction::Update => meta_config_opt.update(),
        ContractCliAction::GenerateSnippets(gs_arg) => {
            meta_config_opt.generate_rust_snippets(&gs_arg);
            meta_config_opt.reload_sc_config();
            meta_config_opt.generate_proxy()
        }
        ContractCliAction::GenerateProxies(proxy_args) => {
            if proxy_args.compare {
                meta_config_opt.compare_proxy()
            } else {
                meta_config_opt.generate_proxy()
            }
        }
        ContractCliAction::ScenBlackbox(args) => {
            scen_blackbox::generate_blackbox_tests(args.overwrite);
        }
    }
}

fn process_original_abi<AbiObj: ContractAbiProvider>(cli_args: &ContractCliArgs) -> MetaConfig {
    let input_abi = <AbiObj as ContractAbiProvider>::abi();
    let mut meta_config = MetaConfig::create(input_abi, cli_args.load_abi_git_version);
    meta_config.sc_config.validate_contract_variants();
    meta_config.write_contract_abis();
    meta_config.write_esdt_attribute_abis();
    meta_config.generate_wasm_crates();
    meta_config
}

pub fn multi_contract_config<AbiObj>(contract_crate_path: &Path) -> ScConfig
where
    AbiObj: ContractAbiProvider,
{
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();

    let sc_config =
        ScConfig::load_from_crate_or_default(contract_crate_path, &original_contract_abi);
    sc_config.validate_contract_variants();
    sc_config
}
