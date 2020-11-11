extern crate non_fungible_tokens;
use non_fungible_tokens::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn _contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/non-fungible-tokens.wasm",
		Box::new(|context| Box::new(NonFungibleTokensImpl::new(context))),
	);
	contract_map
}
