use elrond_wasm_debug::*;
use payable_features::*;

fn main() {
	let contract = PayableFeaturesImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
