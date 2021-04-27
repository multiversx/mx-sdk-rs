use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/ping-pong-egld.wasm",
		Box::new(|context| Box::new(ping_pong_egld::contract_obj(context))),
	);
	contract_map
}

#[test]
fn ping_pong_call_get_user_addresses() {
	parse_execute_mandos(
		"mandos/ping-pong-call-get-user-addresses.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping() {
	parse_execute_mandos("mandos/ping-pong-call-ping.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_ping_after_deadline() {
	parse_execute_mandos(
		"mandos/ping-pong-call-ping-after-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_before_activation() {
	parse_execute_mandos(
		"mandos/ping-pong-call-ping-before-activation.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_second_user() {
	parse_execute_mandos(
		"mandos/ping-pong-call-ping-second-user.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_twice() {
	parse_execute_mandos(
		"mandos/ping-pong-call-ping-twice.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_ping_wrong_ammount() {
	parse_execute_mandos(
		"mandos/ping-pong-call-ping-wrong-ammount.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong() {
	parse_execute_mandos("mandos/ping-pong-call-pong.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_pong_all() {
	parse_execute_mandos("mandos/ping-pong-call-pong-all.scen.json", &contract_map());
}

#[test]
fn ping_pong_call_pong_all_after_pong() {
	parse_execute_mandos(
		"mandos/ping-pong-call-pong-all-after-pong.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_before_deadline() {
	parse_execute_mandos(
		"mandos/ping-pong-call-pong-before-deadline.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_twice() {
	parse_execute_mandos(
		"mandos/ping-pong-call-pong-twice.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_call_pong_without_ping() {
	parse_execute_mandos(
		"mandos/ping-pong-call-pong-without-ping.scen.json",
		&contract_map(),
	);
}

#[test]
fn ping_pong_init() {
	parse_execute_mandos("mandos/ping-pong-init.scen.json", &contract_map());
}
