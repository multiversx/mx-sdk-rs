use adder::*;
use elrond_wasm_debug::*;

fn main() {
	let mut contract_map = ContractMap::<TxContext>::new();
	contract_map.register_contract("file:../output/adder.wasm", Box::new(|context| Box::new(AdderImpl::new(context))));

	parse_execute_mandos("examples/adder/mandos/adder.scen.json", &contract_map);

	println!("Ok");
}
