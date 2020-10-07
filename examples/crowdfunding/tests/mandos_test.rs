
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
fn test_mandos() {
    //parse_execute_mandos("mandos/crowdfunding.scen.json", &contract_map());
}
