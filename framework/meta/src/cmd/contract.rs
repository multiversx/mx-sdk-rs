mod generate_snippets;
mod meta_abi;
mod meta_config;
pub mod sc_config;
pub mod wasm_cargo_toml_data;
pub mod wasm_cargo_toml_generate;

use std::path::Path;

use crate::cli_args::{ContractCliAction, ContractCliArgs};
use clap::Parser;
use meta_config::MetaConfig;
use multiversx_sc::contract_base::ContractAbiProvider;
use sc_config::ScConfig;

/// Entry point in the program from the contract meta crates.
pub fn cli_main<AbiObj: ContractAbiProvider>() {
    let cli_args = ContractCliArgs::parse();
    let mut meta_config_opt = process_original_abi::<AbiObj>(&cli_args);
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
