use super::{
    meta_config::MetaConfig, meta_validate_abi::validate_abi, output_contract::OutputContractConfig,
};
use crate::{
    cli_args::{ContractCliAction, ContractCliArgs, StandaloneCliAction, StandaloneCliArgs},
    local_deps::local_deps,
    meta_all::call_all_meta,
    meta_info::call_info,
    sc_upgrade::upgrade_sc,
};
use clap::Parser;
use multiversx_sc::contract_base::ContractAbiProvider;

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
        None => {},
    }
}

/// Entry point in the program from the contract meta crates.
pub fn cli_main<AbiObj: ContractAbiProvider>() {
    let cli_args = ContractCliArgs::parse();
    let mut meta_config_opt = process_abi::<AbiObj>(&cli_args);
    match cli_args.command {
        ContractCliAction::Abi => {},
        ContractCliAction::Build(build_args) => meta_config_opt.build(build_args),
        ContractCliAction::BuildDbg(build_args) => {
            meta_config_opt.build(build_args.into_build_args())
        },
        ContractCliAction::Twiggy(build_args) => {
            meta_config_opt.build(build_args.into_build_args())
        },
        ContractCliAction::Clean => meta_config_opt.clean(),
        ContractCliAction::Update => meta_config_opt.update(),
        ContractCliAction::GenerateSnippets(gs_args) => {
            meta_config_opt.generate_rust_snippets(&gs_args)
        },
    }
}

fn process_abi<AbiObj: ContractAbiProvider>(cli_args: &ContractCliArgs) -> MetaConfig {
    let input_abi = <AbiObj as ContractAbiProvider>::abi();
    validate_abi(&input_abi).expect("Invalid contract structure");
    let mut meta_config = MetaConfig::create(input_abi, cli_args.load_abi_git_version);
    meta_config.write_abi();
    meta_config.generate_wasm_crates();
    meta_config
}

pub fn multi_contract_config<AbiObj: ContractAbiProvider>(
    multi_contract_config_toml_path: &str,
) -> OutputContractConfig {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    validate_abi(&original_contract_abi).expect("Invalid contract structure");

    OutputContractConfig::load_from_file(multi_contract_config_toml_path, &original_contract_abi)
        .unwrap_or_else(|| panic!("could not find file {multi_contract_config_toml_path}"))
}
