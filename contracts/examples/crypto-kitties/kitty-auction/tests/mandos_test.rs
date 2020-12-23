extern crate kitty_auction;
use kitty_auction::*;

extern crate kitty_ownership;
use kitty_ownership::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();

	contract_map.register_contract(
		"file:../../kitty-ownership/output/kitty-ownership.wasm",
		Box::new(|context| Box::new(KittyOwnershipImpl::new(context))),
	);
	contract_map.register_contract(
		"file:../output/kitty-auction.wasm",
		Box::new(|context| Box::new(KittyAuctionImpl::new(context))),
	);

	contract_map
}

#[test]
fn init() {
	parse_execute_mandos("mandos/init.scen.json", &contract_map());
}

#[test]
fn create_and_auction_gen_zero_kitty() {
	parse_execute_mandos("mandos/create_and_auction_gen_zero_kitty.scen.json", &contract_map());
}
