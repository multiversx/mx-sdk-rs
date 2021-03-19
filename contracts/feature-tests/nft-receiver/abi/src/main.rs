use nft_receiver::*;
use elrond_wasm_debug::*;

fn main() {
	let contract = NftReceiverImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
