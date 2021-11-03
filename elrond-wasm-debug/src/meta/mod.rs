mod meta_abi;
mod meta_build_wasm;
mod meta_validate_abi;
mod meta_wasm_src;

use std::env;

use elrond_wasm::contract_base::ContractAbiProvider;

pub fn perform<AbiObj: ContractAbiProvider>() {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    meta_validate_abi::validate_abi(&abi).unwrap();
    meta_abi::write_abi(&abi);
    meta_wasm_src::write_wasm_lib(&abi);
    meta_wasm_src::copy_to_wasm_unmanaged_ei();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "build" => meta_build_wasm::build_wasm(&abi, args.as_slice()),
            "clean" => meta_build_wasm::clean_wasm(),
            _ => (),
        }
    }
}
