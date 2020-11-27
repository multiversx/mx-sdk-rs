extern crate kitty_ownership;
use kitty_ownership::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

/*fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/non-fungible-tokens.wasm",
		Box::new(|context| Box::new(NonFungibleTokensImpl::new(context))),
	);
	contract_map
}

#[test]
fn nft_init() {
	parse_execute_mandos("mandos/nft-init.scen.json", &contract_map());
}*/
