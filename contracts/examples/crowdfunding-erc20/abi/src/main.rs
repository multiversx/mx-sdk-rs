use crowdfunding_erc20::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = CrowdfundingImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
