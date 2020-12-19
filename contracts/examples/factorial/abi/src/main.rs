use factorial::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = FactorialImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
