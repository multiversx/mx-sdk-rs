use elrond_wasm_debug::*;

fn main() {
	let contract = use_module::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
