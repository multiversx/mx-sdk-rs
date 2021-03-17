use esdt_nft_marketplace::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = EsdtNftMarketplaceImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
