use elrond_wasm_debug::*;
use esdt_nft_marketplace::*;

fn main() {
	let contract = EsdtNftMarketplaceImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
