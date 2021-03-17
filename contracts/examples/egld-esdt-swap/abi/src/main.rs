use egld_esdt_swap::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = EgldEsdtSwapImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
