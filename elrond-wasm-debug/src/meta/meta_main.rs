use elrond_wasm::contract_base::ContractAbiProvider;
use std::env;

use super::meta_config::MetaConfig;

pub fn perform<AbiObj: ContractAbiProvider>() {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    let args: Vec<String> = env::args().collect();
    let mut meta_config = MetaConfig::create(&original_contract_abi, args.as_slice());

    meta_config.write_abi();

    meta_config.create_wasm_view_cargo_toml();

    meta_config.write_wasm_src_lib();

    super::meta_wasm_crates::copy_to_wasm_unmanaged_ei();

    if args.len() > 1 {
        match args[1].as_str() {
            "build" => meta_config.build_wasm(),
            "clean" => meta_config.clean_wasm(),
            _ => (),
        }
    }
}
