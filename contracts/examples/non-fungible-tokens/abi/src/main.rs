use elrond_wasm_debug::*;

fn main() {
	let contract = non_fungible_tokens::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
