use elrond_wasm::CallableContract;
use elrond_wasm_debug::*;
use use_module::*;

fn main() {
	let contract = UseModuleImpl::new(TxContext::dummy());
	let abi = contract.abi(true);
	let json = elrond_wasm_debug::abi_json::serialize_abi_to_json(&abi);
	print!("{}", json);
}
