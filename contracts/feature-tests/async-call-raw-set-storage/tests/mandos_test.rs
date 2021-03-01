extern crate first_contract_async;
use first_contract_async::*;
extern crate second_contract_async;
use second_contract_async::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../first-contract-async/output/first-contract-async.wasm",
		Box::new(|context| Box::new(FirstContractAsyncImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../second-contract-async/output/second-contract-async.wasm",
		Box::new(|context| Box::new(SecondContractAsyncImpl::new(context))),
	);
	contract_map
}

#[test]
fn init() {
	parse_execute_mandos("mandos/init.scen.json", &contract_map());
}

#[test]
fn async_call_raw_set_storage() {
	parse_execute_mandos("mandos/async-call-raw-set-storage.scen.json", &contract_map());
}
