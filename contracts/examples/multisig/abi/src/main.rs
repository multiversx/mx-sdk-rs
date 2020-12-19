use elrond_wasm_debug::*;
use multisig::*;

fn main() {
	let contract = MultisigImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
