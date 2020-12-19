use elrond_wasm_debug::*;
use lottery_esdt::*;

fn main() {
	let contract = LotteryImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
