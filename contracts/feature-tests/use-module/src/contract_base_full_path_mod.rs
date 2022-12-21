mx_sc::imports!();

#[mx_sc::module]
pub trait ContractBaseFullPathTestModule: mx_sc::contract_base::ContractBase {
    #[endpoint]
    fn call_contract_base_full_path_endpoint(&self) {}
}
