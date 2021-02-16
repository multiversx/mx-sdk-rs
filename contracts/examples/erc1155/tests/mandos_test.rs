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
fn deploy_test() {
	parse_execute_mandos("mandos/deploy.scen.json", &contract_map());
}

// Create token tests

#[test]
fn create_token_fungible_test() {
	parse_execute_mandos("mandos/create_token_fungible.scen.json", &contract_map());
}

#[test]
fn create_token_non_fungible_test() {
	parse_execute_mandos("mandos/create_token_non_fungible.scen.json", &contract_map());
}

#[test]
fn create_two_fungible_same_creator_test() {
	parse_execute_mandos("mandos/create_two_tokens_both_fungible_same_creator.scen.json", &contract_map());
}

#[test]
fn create_two_fungible_different_creator_test() {
    parse_execute_mandos("mandos/create_two_tokens_both_fungible_different_creator.scen.json", &contract_map());
}

#[test]
fn create_two_non_fungible_same_creator_test() {
	parse_execute_mandos("mandos/create_two_tokens_both_non_fungible_same_creator.scen.json", &contract_map());
}

#[test]
fn create_one_fungible_one_non_fungible_test() {
	parse_execute_mandos("mandos/create_one_fungible_one_non_fungible.scen.json", &contract_map());
}
