use adder::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = AdderImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
