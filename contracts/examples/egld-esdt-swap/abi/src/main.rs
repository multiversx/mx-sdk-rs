use elrond_wasm_debug::*;

fn main() {
	let contract = egld_esdt_swap::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
