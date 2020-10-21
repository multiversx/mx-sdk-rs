
extern crate lottery;
use lottery::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/lottery.wasm",
        Box::new(|context| Box::new(LotteryImpl::new(context))));
    contract_map
}

#[test]
fn test_mandos() {
    parse_execute_mandos("mandos/start-limited-tickets.scen.json", &contract_map());
}
