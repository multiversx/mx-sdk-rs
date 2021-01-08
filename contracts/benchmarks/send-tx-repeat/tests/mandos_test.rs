extern crate send_tx_repeat;
use elrond_wasm::*;
use elrond_wasm_debug::*;
use send_tx_repeat::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/send-tx-repeat.wasm",
		Box::new(|context| Box::new(SendTxRepeatImpl::new(context))),
	);
	contract_map
}

#[test]
fn test_send_tx_repeat_without_data_mandos() {
	parse_execute_mandos("mandos/send_tx_repeat_without_data.scen.json", &contract_map());
}

#[test]
fn test_send_tx_repeat_with_data_mandos() {
	parse_execute_mandos("mandos/send_tx_repeat_with_data.scen.json", &contract_map());
}
