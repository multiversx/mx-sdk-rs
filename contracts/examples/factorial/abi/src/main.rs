use elrond_wasm_debug::*;
use factorial::*;

fn main() {
	let contract = FactorialImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
