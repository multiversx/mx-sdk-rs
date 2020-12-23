extern crate kitty_ownership;
use kitty_ownership::*;

extern crate kitty_genetic_alg;
use kitty_genetic_alg::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-genetic-alg/output/kitty-genetic-alg.wasm",
		Box::new(|context| Box::new(KittyGeneticAlgImpl::new(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-ownership.wasm",
		Box::new(|context| Box::new(KittyOwnershipImpl::new(context))),
	);

	contract_map
}

#[test]
fn init() {
	parse_execute_mandos("mandos/init.scen.json", &contract_map());
}

#[test]
fn setup_accounts() {
	parse_execute_mandos("mandos/setup_accounts.scen.json", &contract_map());
}

#[test]
fn query() {
	parse_execute_mandos("mandos/query.scen.json", &contract_map());
}
