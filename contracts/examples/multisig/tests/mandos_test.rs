extern crate multisig;
use elrond_wasm::*;
use elrond_wasm_debug::*;
use multisig::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/multisig.wasm",
		Box::new(|context| Box::new(MultisigImpl::new(context))),
	);
	contract_map
}

#[test]
fn test_mandos() {
	parse_execute_mandos("mandos/deploy.scen.json", &contract_map());
}
