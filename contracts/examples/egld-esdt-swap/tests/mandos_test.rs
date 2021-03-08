extern crate egld_esdt_swap;
use egld_esdt_swap::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/egld-esdt-swap.wasm",
		Box::new(|context| Box::new(EgldEsdtSwapImpl::new(context))),
	);
	contract_map
}

#[test]
fn wrap_egld_test() {
	parse_execute_mandos("mandos/wrap_egld.scen.json", &contract_map());
}

#[test]
fn wrap_then_unwrap_egld_test() {
	parse_execute_mandos("mandos/unwrap_egld.scen.json", &contract_map());
}
