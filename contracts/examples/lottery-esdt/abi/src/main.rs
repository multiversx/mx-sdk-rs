use elrond_wasm_debug::*;

fn main() {
	let contract = lottery_esdt::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
