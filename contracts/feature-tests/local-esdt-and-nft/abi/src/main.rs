use local_esdt_and_nft::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = LocalEsdtAndEsdtNftImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
