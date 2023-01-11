use multiversx_sc::contract_base::ContractAbiProvider;
use std::env;

use crate::cli_args::{CliAction, CliArgs};

use super::{meta_config::MetaConfig, output_contract::OutputContractConfig};

pub fn cli_main<AbiObj: ContractAbiProvider>() {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    let args: Vec<String> = env::args().collect();
    let cli_args = CliArgs::parse(args.as_slice()).expect("Error processing CLI arguments: ");
    let mut meta_config = MetaConfig::create(original_contract_abi, cli_args.load_abi_git_version);

    meta_config.write_abi();

    meta_config.generate_wasm_crates();

    match cli_args.action {
        CliAction::Nothing => {},
        CliAction::Build(build_args) => meta_config.build(build_args),
        CliAction::Clean => meta_config.clean(),
        CliAction::GenerateSnippets(gs_args) => meta_config.generate_rust_snippets(&gs_args),
    }
}

pub fn multi_contract_config<AbiObj: ContractAbiProvider>(
    multi_contract_config_toml_path: &str,
) -> OutputContractConfig {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    OutputContractConfig::load_from_file(multi_contract_config_toml_path, &original_contract_abi)
        .unwrap_or_else(|| panic!("could not find file {multi_contract_config_toml_path}"))
}
