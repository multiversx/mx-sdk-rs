mod meta_abi;
mod meta_validate_abi;
mod meta_wasm_src;

use elrond_wasm::contract_base::ContractAbiProvider;

pub fn perform<AbiObj: ContractAbiProvider>() {
    let abi = <AbiObj as ContractAbiProvider>::abi();
    meta_validate_abi::validate_abi(&abi).unwrap();
    meta_abi::write_abi(&abi);
    meta_wasm_src::write_wasm_lib(&abi);
    meta_wasm_src::copy_to_wasm_unmanaged_ei();
}
