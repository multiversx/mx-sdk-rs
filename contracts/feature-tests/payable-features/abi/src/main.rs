use payable_features::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = PayableFeaturesImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
