use super::{meta_config::MetaConfig, output_contract::OutputContractConfig};
use crate::cli_args::{CliAction, CliArgs};
use multiversx_sc::{
    abi::ContractAbi, api::uncallable::UncallableApi, contract_base::ContractAbiProvider,
};
use std::env;

/// The ABI provider set when
pub struct NoAbiProvider;

impl NoAbiProvider {
    pub const NAME: &str = "no-abi";
}

impl ContractAbiProvider for NoAbiProvider {
    type Api = UncallableApi;

    fn abi() -> ContractAbi {
        ContractAbi {
            name: Self::NAME,
            ..Default::default()
        }
    }
}

fn process_abi<AbiObj: ContractAbiProvider>(cli_args: &CliArgs) -> Option<MetaConfig> {
    let input_abi = <AbiObj as ContractAbiProvider>::abi();
    if input_abi.name == NoAbiProvider::NAME {
        return None;
    }

    super::meta_validate_abi::validate_abi(&input_abi).unwrap();
    let mut meta_config = MetaConfig::create(input_abi, cli_args.load_abi_git_version);
    meta_config.write_abi();
    meta_config.generate_wasm_crates();
    Some(meta_config)
}

pub fn cli_main<AbiObj: ContractAbiProvider>() {
    let args: Vec<String> = env::args().collect();
    let cli_args = CliArgs::parse(args.as_slice()).expect("Error processing CLI arguments: ");

    let meta_config_opt = process_abi::<AbiObj>(&cli_args);

    match cli_args.action {
        CliAction::Nothing => {},
        CliAction::Build(build_args) => meta_config_opt
            .expect("cannot call build in the general meta tool")
            .build(build_args),
        CliAction::Clean => meta_config_opt
            .expect("cannot call clean in the general meta tool")
            .clean(),
        CliAction::GenerateSnippets(gs_args) => meta_config_opt
            .expect("cannot call snippets in the general meta tool")
            .generate_rust_snippets(&gs_args),
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
