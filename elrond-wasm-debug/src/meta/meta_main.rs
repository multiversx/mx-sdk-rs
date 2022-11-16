use elrond_wasm::contract_base::ContractAbiProvider;
use std::env;

use super::{meta_config::MetaConfig, multi_contract::MultiContract};

static SNIPPETS_OVERWRITE_FLAG_NAME: &str = "--overwrite";

pub fn perform<AbiObj: ContractAbiProvider>() {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    let args: Vec<String> = env::args().collect();
    let arguments = args.as_slice();
    let mut meta_config;

    if let Some(contract_details) = get_multicontract(){
        meta_config = MetaConfig::interpret_toml(&original_contract_abi, arguments, contract_details);
    }
    else {    
        meta_config = MetaConfig::create(&original_contract_abi, arguments);
    }

    meta_config.write_abi();

    meta_config.create_wasm_secondary_cargo_toml();

    meta_config.write_wasm_src_lib();

    super::meta_wasm_crates::copy_to_wasm_unmanaged_ei();

    if args.len() > 1 {
        match args[1].as_str() {
            "build" => meta_config.build_wasm(),
            "clean" => meta_config.clean_wasm(),
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

pub fn get_multicontract() -> Option<MultiContract>{
    if let Ok(content) = std::fs::read_to_string("./multicontract.toml"){
        if let Ok(multicontract) = toml::from_str(&content){
            return Some(multicontract);
        }
    }
    panic!("Hopa!");
    return None;
}
