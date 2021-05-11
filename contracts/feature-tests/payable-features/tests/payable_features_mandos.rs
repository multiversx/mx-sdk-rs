use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/payable-features.wasm",
		Box::new(|context| Box::new(payable_features::contract_obj(context))),
	);
	contract_map
}

#[test]
fn payable_any_1() {
	parse_execute_mandos("mandos/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2() {
	parse_execute_mandos("mandos/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3() {
	parse_execute_mandos("mandos/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4() {
	parse_execute_mandos("mandos/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_egld_0() {
	parse_execute_mandos("mandos/payable_egld_0.scen.json", &contract_map());
}

#[test]
fn payable_egld_1() {
	parse_execute_mandos("mandos/payable_egld_1.scen.json", &contract_map());
}

#[test]
fn payable_egld_2() {
	parse_execute_mandos("mandos/payable_egld_2.scen.json", &contract_map());
}

#[test]
fn payable_egld_3() {
	parse_execute_mandos("mandos/payable_egld_3.scen.json", &contract_map());
}

#[test]
fn payable_egld_4() {
	parse_execute_mandos("mandos/payable_egld_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1() {
	parse_execute_mandos("mandos/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2() {
	parse_execute_mandos("mandos/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3() {
	parse_execute_mandos("mandos/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4() {
	parse_execute_mandos("mandos/payable_token_4.scen.json", &contract_map());
}
