extern crate erc1155;
use erc1155::*;

extern crate erc1155_marketplace;
use erc1155_marketplace::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/erc1155-marketplace.wasm",
		Box::new(|context| Box::new(Erc1155MarketplaceImpl::new(context))),
	);
	contract_map.register_contract(
		"file:../../erc1155/output/erc1155.wasm",
		Box::new(|context| Box::new(Erc1155Impl::new(context))),
	);

	contract_map
}

#[test]
fn auction_single_token_egld_test() {
	parse_execute_mandos("mandos/auction_single_token_egld.scen.json", &contract_map());
}

#[test]
fn auction_batch_test() {
	parse_execute_mandos("mandos/auction_batch.scen.json", &contract_map());
}

#[test]
fn bid_first_egld_test() {
	parse_execute_mandos("mandos/bid_first_egld.scen.json", &contract_map());
}

#[test]
fn bid_second_egld_test() {
	parse_execute_mandos("mandos/bid_second_egld.scen.json", &contract_map());
}

#[test]
fn bid_third_egld_test() {
	parse_execute_mandos("mandos/bid_third_egld.scen.json", &contract_map());
}

#[test]
fn end_auction_test() {
	parse_execute_mandos("/home/elrond/elrond-wasm-rs/contracts/examples/erc1155-marketplace/mandos/end_auction.scen.json", &contract_map());
}
