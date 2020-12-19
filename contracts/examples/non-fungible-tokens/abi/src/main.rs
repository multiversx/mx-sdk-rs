use non_fungible_tokens::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = NonFungibleTokensImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
