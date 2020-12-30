use elrond_wasm_debug::*;
use lottery_egld::*;

fn main() {
	let contract = LotteryImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
