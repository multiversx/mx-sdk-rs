extern crate simple_erc20;
use simple_erc20::*;

extern crate crowdfunding;
use crowdfunding::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();

    contract_map.register_contract(
        "file:../output/simple-erc20.wasm",
        Box::new(|context| Box::new(SimpleErc20TokenImpl::new(context))));

    contract_map.register_contract(
        "file:../../crowdfunding-egld/output/crowdfunding.wasm",
        Box::new(|context| Box::new(CrowdfundingImpl::new(context))));
        

    contract_map
}

#[test]
fn deploy_erc20_and_crowdfunding() {
    parse_execute_mandos("mandos/deploy_erc20_and_crowdfunding.scen.json", &contract_map());
}
