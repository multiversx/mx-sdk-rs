use elrond_wasm_debug::*;

fn main() {
	let contract = multisig::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
