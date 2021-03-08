use erc1155_user_mock::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = Erc1155UserMockImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
