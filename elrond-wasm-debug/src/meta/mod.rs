mod meta_abi;
mod meta_wasm_src;

use elrond_wasm::contract_base::ContractAbiProvider;

pub fn perform<AbiObj: ContractAbiProvider>() {
    meta_abi::write_abi::<AbiObj>();
    meta_wasm_src::write_wasm_lib::<AbiObj>();
}
