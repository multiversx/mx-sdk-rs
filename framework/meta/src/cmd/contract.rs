mod generate_snippets;
mod meta_abi;
mod meta_config;
pub mod output_contract;

mod validate_abi;

use crate::cli_args::{ContractCliAction, ContractCliArgs};
use clap::Parser;
use meta_config::MetaConfig;
use multiversx_sc::contract_base::ContractAbiProvider;
use output_contract::OutputContractGlobalConfig;
use validate_abi::validate_abi;

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
) -> OutputContractGlobalConfig {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    validate_abi(&original_contract_abi).expect("Invalid contract structure");

    OutputContractGlobalConfig::load_from_file(
        multi_contract_config_toml_path,
        &original_contract_abi,
    )
    .unwrap_or_else(|| panic!("could not find file {multi_contract_config_toml_path}"))
}
