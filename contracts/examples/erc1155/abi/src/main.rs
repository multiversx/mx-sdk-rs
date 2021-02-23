use erc1155::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = Erc1155Impl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
