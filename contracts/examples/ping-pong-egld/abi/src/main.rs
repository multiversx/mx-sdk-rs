use elrond_wasm_debug::*;

fn main() {
	let contract = ping_pong_egld::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
