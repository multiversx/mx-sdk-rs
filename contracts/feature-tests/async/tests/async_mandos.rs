use async_alice::*;
use async_bob::*;
use forwarder::*;
use forwarder_raw::*;
use vault::*;

use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../async-alice/output/async-alice.wasm",
		Box::new(|context| Box::new(AliceImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../async-bob/output/async-bob.wasm",
		Box::new(|context| Box::new(BobImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../forwarder/output/forwarder.wasm",
		Box::new(|context| Box::new(ForwarderImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../forwarder-raw/output/forwarder-raw.wasm",
		Box::new(|context| Box::new(ForwarderRawImpl::new(context))),
	);

	contract_map.register_contract(
		"file:../vault/output/vault.wasm",
		Box::new(|context| Box::new(VaultImpl::new(context))),
	);

	contract_map
}

// #[test]
// fn forwarder_async_accept_egld() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_async_accept_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_async_accept_esdt() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_async_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_accept_egld() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_raw_async_accept_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_accept_esdt() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_raw_async_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_async_echo() {
// 	parse_execute_mandos("mandos/forwarder_raw_async_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_raw_direct_egld() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_raw_direct_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_direct_esdt() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_raw_direct_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_raw_sync_echo() {
// 	parse_execute_mandos("mandos/forwarder_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_raw_sync_egld() {
// 	parse_execute_mandos("mandos/forwarder_raw_sync_egld.scen.json", &contract_map());
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_egld() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_send_twice_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// TODO: successive asyncs currently not supported
// #[test]
// fn forwarder_send_twice_esdt() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_send_twice_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_egld() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_sync_accept_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_accept_esdt() {
// 	parse_execute_mandos(
// 		"mandos/forwarder_sync_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_echo() {
// 	parse_execute_mandos("mandos/forwarder_sync_echo.scen.json", &contract_map());
// }

#[test]
fn message_othershard() {
	parse_execute_mandos("mandos/message_otherShard.scen.json", &contract_map());
}

#[test]
fn message_othershard_callback() {
	parse_execute_mandos(
		"mandos/message_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn message_sameshard() {
	parse_execute_mandos("mandos/message_sameShard.scen.json", &contract_map());
}

#[test]
fn message_sameshard_callback() {
	parse_execute_mandos(
		"mandos/message_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_othershard() {
	parse_execute_mandos("mandos/payment_otherShard.scen.json", &contract_map());
}

#[test]
fn payment_othershard_callback() {
	parse_execute_mandos(
		"mandos/payment_otherShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn payment_sameshard() {
	parse_execute_mandos("mandos/payment_sameShard.scen.json", &contract_map());
}

#[test]
fn payment_sameshard_callback() {
	parse_execute_mandos(
		"mandos/payment_sameShard_callback.scen.json",
		&contract_map(),
	);
}

#[test]
fn send_egld() {
	parse_execute_mandos("mandos/send_egld.scen.json", &contract_map());
}

#[test]
fn send_esdt() {
	parse_execute_mandos("mandos/send_esdt.scen.json", &contract_map());
}
