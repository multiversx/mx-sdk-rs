use simple_erc20::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = SimpleErc20TokenImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
