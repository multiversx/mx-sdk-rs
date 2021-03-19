use erc1155_marketplace::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = Erc1155MarketplaceImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
