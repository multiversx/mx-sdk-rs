
extern crate crowdfunding;
use crowdfunding::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/crowdfunding.wasm",
        Box::new(|context| Box::new(CrowdfundingImpl::new(context))));
    contract_map
}

#[test]
fn test_crowdfunding1() {
parse_execute_mandos("mandos/crowdfunding-fund-too-late.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding2() {
parse_execute_mandos("mandos/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding3() {
parse_execute_mandos("mandos/crowdfunding-init.scen.json", &contract_map());
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
