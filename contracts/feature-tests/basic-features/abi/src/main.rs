use basic_features::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = BasicFeaturesImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
