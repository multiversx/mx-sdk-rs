use crowdfunding::*;
use elrond_wasm_debug::*;

fn main() {
    let mut contract_map = ContractMap::<TxContext>::new();
    contract_map.register_contract(
        "file:../output/crowdfunding.wasm",
        Box::new(|context| Box::new(AdderImpl::new(context))));

    parse_execute_mandos("examples/crowdfunding/mandos/crowdfunding.scen.json", &contract_map);
    
    println!("Ok");
}
