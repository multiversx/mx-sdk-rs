
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

/*#[test]
fn test_crowdfunding1() {
parse_execute_mandos("mandos/crowdfunding-fund-too-late.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding2() {
parse_execute_mandos("mandos/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding4() {
    parse_execute_mandos("mandos/crowdfunding-claim-too-early.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding5() {
    parse_execute_mandos("mandos/crowdfunding-claim-successful.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding6() {
    parse_execute_mandos("mandos/crowdfunding-claim-failed.scen.json", &contract_map());
}
*/