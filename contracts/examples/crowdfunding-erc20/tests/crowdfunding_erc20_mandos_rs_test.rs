use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<DebugApi> {
    let mut contract_map = ContractMap::new();

    contract_map.register_contract(
        "file:output/crowdfunding-erc20.wasm",
        Box::new(|context| Box::new(crowdfunding_erc20::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../erc20/output/erc20.wasm",
        Box::new(|context| Box::new(erc20::contract_obj(context))),
    );

    contract_map
}

#[test]
fn deploy_erc20_and_crowdfunding_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/deploy_erc20_and_crowdfunding.scen.json",
        contract_map(),
    );
}

#[test]
fn fund_with_insufficient_allowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/fund_with_insufficient_allowance.scen.json",
        contract_map(),
    );
}

#[test]
fn fund_with_sufficient_allowance_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/fund_with_sufficient_allowance.scen.json",
        contract_map(),
    );
}

#[test]
fn fund_without_allowance_rs() {
    elrond_wasm_debug::mandos_rs("mandos/fund_without_allowance.scen.json", contract_map());
}
