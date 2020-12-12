use adder::*;
use elrond_wasm::CallableContract;
use elrond_wasm_debug::*;

fn main() {
	let adder = AdderImpl::new(TxContext::dummy());
	let abi = adder.abi(true);
	let json = elrond_wasm_debug::abi_json::serialize_abi_to_json(&abi);
	print!("{}", json);
}
