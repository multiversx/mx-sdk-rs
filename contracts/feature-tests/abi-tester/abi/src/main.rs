use elrond_wasm_debug::*;

fn main() {
	let contract = abi_tester::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
