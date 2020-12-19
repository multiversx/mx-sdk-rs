use elrond_wasm_debug::*;
use non_fungible_tokens::*;

fn main() {
	let contract = NonFungibleTokensImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
