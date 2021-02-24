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
fn auction_single_token_test() {
	parse_execute_mandos("mandos/auction_single_token.scen.json", &contract_map());
}
