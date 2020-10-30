
extern crate simple_erc20;
use simple_erc20::*;

extern crate crowdfunding_erc20;
use crowdfunding_erc20::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();

    contract_map.register_contract(
        "file:../output/crowdfunding-erc20.wasm",
        Box::new(|context| Box::new(CrowdfundingImpl::new(context))));

    contract_map.register_contract(
        "file:../../simple-erc20/output/simple-erc20.wasm",
        Box::new(|context| Box::new(SimpleErc20TokenImpl::new(context))));
        

    contract_map
}

#[test]
fn deploy_erc20_and_crowdfunding() {
    parse_execute_mandos("mandos/deploy_erc20_and_crowdfunding.scen.json", &contract_map());
}

#[test]
fn fund_without_allowance() {
    parse_execute_mandos("mandos/fund_without_allowance.scen.json", &contract_map());
}

#[test]
fn fund_with_insufficient_allowance() {
    parse_execute_mandos("mandos/fund_with_insufficient_allowance.scen.json", &contract_map());
}

#[test]
fn fund_with_sufficient_allowance() {
    parse_execute_mandos("mandos/fund_with_sufficient_allowance.scen.json", &contract_map());
}
