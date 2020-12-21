use crowdfunding_egld::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = CrowdfundingImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
