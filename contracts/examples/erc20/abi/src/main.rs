use elrond_wasm_debug::*;

fn main() {
	let contract = erc20::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
