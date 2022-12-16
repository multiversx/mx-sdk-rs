use elrond_wasm::contract_base::ContractAbiProvider;
use std::env;

use super::{
    meta_build_args::BuildArgs, meta_config::MetaConfig, output_contract::OutputContractConfig,
};

static SNIPPETS_OVERWRITE_FLAG_NAME: &str = "--overwrite";

pub fn perform<AbiObj: ContractAbiProvider>() {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    let args: Vec<String> = env::args().collect();
    let build_args = BuildArgs::process(args.as_slice());
    let mut meta_config = MetaConfig::create(original_contract_abi, build_args);

    meta_config.write_abi();

    meta_config.generate_wasm_crates();

    if args.len() > 1 {
        match args[1].as_str() {
            "build" => meta_config.build(),
            "build-dbg" => meta_config.build_dbg(),
            "clean" => meta_config.clean(),
            "snippets" => {
                let overwrite = match args.get(2) {
                    Some(arg) => arg.as_str() == SNIPPETS_OVERWRITE_FLAG_NAME,
                    None => false,
                };

                meta_config.generate_rust_snippets(overwrite);
            },
            _ => (),
        }
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
