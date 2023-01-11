multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ContractBaseFullPathTestModule: multiversx_sc::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
