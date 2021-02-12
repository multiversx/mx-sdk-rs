extern crate erc1155;
use erc1155::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc1155.wasm",
		Box::new(|context| Box::new(Erc1155Impl::new(context))),
	);
	contract_map
}

#[test]
fn test_mandos() {
	parse_execute_mandos("mandos/", &contract_map());
}
