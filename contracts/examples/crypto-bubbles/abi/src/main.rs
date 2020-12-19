use crypto_bubbles::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = CryptoBubblesImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
