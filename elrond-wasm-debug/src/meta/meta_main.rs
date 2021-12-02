use elrond_wasm::{abi::EndpointLocationAbi, contract_base::ContractAbiProvider};
use std::env;

pub fn perform<AbiObj: ContractAbiProvider>() {
    let original_contract_abi = <AbiObj as ContractAbiProvider>::abi();
    super::meta_validate_abi::validate_abi(&original_contract_abi).unwrap();

    let main_contract_abi = original_contract_abi.main_contract();
    super::meta_abi::write_abi(&main_contract_abi, "");
    super::meta_wasm_src::write_wasm_lib(&main_contract_abi);
    super::meta_wasm_src::copy_to_wasm_unmanaged_ei();

    let view_contract_abi = original_contract_abi.secondary_contract(EndpointLocationAbi::ViewContract);
    super::meta_abi::write_abi(&view_contract_abi, "-view");

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "build" => super::meta_build_wasm::build_wasm(&main_contract_abi, args.as_slice()),
            "clean" => super::meta_build_wasm::clean_wasm(),
            _ => (),
        }
    }
}
