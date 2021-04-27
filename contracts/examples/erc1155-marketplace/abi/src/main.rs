use elrond_wasm_debug::*;

fn main() {
	let contract = erc1155_marketplace::contract_obj(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
