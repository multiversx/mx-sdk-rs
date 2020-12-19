use elrond_wasm_debug::*;
use use_module::*;

fn main() {
	let contract = UseModuleImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
