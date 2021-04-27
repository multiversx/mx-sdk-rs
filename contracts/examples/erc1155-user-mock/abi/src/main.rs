use elrond_wasm_debug::*;

fn main() {
	let contract = erc1155_user_mock::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
