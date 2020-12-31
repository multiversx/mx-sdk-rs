use elrond_wasm_debug::*;
use ping_pong_egld::*;

fn main() {
	let contract = PingPongImpl::new(TxContext::dummy());
	print!("{}", abi_json::contract_abi(&contract));
}
