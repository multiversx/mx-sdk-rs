elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait ContractBaseFullPathTestModule: elrond_wasm::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
