use basic_features::*;
use elrond_wasm::CallableContract;
use elrond_wasm_debug::*;

fn main() {
	let contract = BasicFeaturesImpl::new(TxContext::dummy());
	let abi = contract.abi(true);
	let json = elrond_wasm_debug::abi_json::serialize_abi_to_json(&abi);
	print!("{}", json);
}
