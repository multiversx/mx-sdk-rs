use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../async-alice/output/async-alice.wasm",
		Box::new(|context| Box::new(async_alice::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../async-bob/output/async-bob.wasm",
		Box::new(|context| Box::new(async_bob::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../forwarder/output/forwarder.wasm",
		Box::new(|context| Box::new(forwarder::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../forwarder-raw/output/forwarder-raw.wasm",
		Box::new(|context| Box::new(forwarder_raw::contract_obj(context))),
	);

	contract_map.register_contract(
		"file:../vault/output/vault.wasm",
		Box::new(|context| Box::new(vault::contract_obj(context))),
	);

	contract_map
}

#[test]
fn forw_raw_async_accept_egld() {
	parse_execute_mandos(
		"mandos/forw_raw_async_accept_egld.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn forw_raw_async_accept_esdt() {
//     parse_execute_mandos("mandos/forw_raw_async_accept_esdt.scen.json", &contract_map());
// }

#[test]
fn forw_raw_async_echo() {
	parse_execute_mandos("mandos/forw_raw_async_echo.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_egld() {
	parse_execute_mandos("mandos/forw_raw_direct_egld.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_esdt() {
	parse_execute_mandos("mandos/forw_raw_direct_esdt.scen.json", &contract_map());
}

// #[test]
// fn forw_raw_sync_echo() {
//     parse_execute_mandos("mandos/forw_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forw_raw_sync_egld() {
//     parse_execute_mandos("mandos/forw_raw_sync_egld.scen.json", &contract_map());
// }

#[test]
fn forwarder_call_async_accept_egld() {
	parse_execute_mandos(
		"mandos/forwarder_call_async_accept_egld.scen.json",
		&contract_map(),
	);
}

// #[test]
// fn forwarder_call_async_accept_esdt() {
//     parse_execute_mandos("mandos/forwarder_call_async_accept_esdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_async_accept_nft() {
//     parse_execute_mandos("mandos/forwarder_call_async_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_egld() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_egld.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_esdt() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_esdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_nft() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_egld() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_then_read_egld.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_esdt() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_then_read_esdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_nft() {
//     parse_execute_mandos("mandos/forwarder_call_sync_accept_then_read_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_egld() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_egld.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_egld_twice() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_egld_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_esdt() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_esdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_esdt_twice() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_esdt_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_nft() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_nft.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_call_transf_exec_accept_sft_twice() {
//     parse_execute_mandos("mandos/forwarder_call_transf_exec_accept_sft_twice.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_create() {
//     parse_execute_mandos("mandos/forwarder_nft_create.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_async() {
//     parse_execute_mandos("mandos/forwarder_nft_transfer_async.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_exec() {
//     parse_execute_mandos("mandos/forwarder_nft_transfer_exec.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_send_twice_egld() {
//     parse_execute_mandos("mandos/forwarder_send_twice_egld.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_send_twice_esdt() {
//     parse_execute_mandos("mandos/forwarder_send_twice_esdt.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo() {
//     parse_execute_mandos("mandos/forwarder_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo_range() {
//     parse_execute_mandos("mandos/forwarder_sync_echo_range.scen.json", &contract_map());
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

// #[test]
// fn recursive_caller_egld_1() {
//     parse_execute_mandos("mandos/recursive_caller_egld_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_egld_2() {
//     parse_execute_mandos("mandos/recursive_caller_egld_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_egld_x() {
//     parse_execute_mandos("mandos/recursive_caller_egld_x.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_1() {
//     parse_execute_mandos("mandos/recursive_caller_esdt_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_2() {
//     parse_execute_mandos("mandos/recursive_caller_esdt_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_x() {
//     parse_execute_mandos("mandos/recursive_caller_esdt_x.scen.json", &contract_map());
// }

#[test]
fn send_egld() {
	parse_execute_mandos("mandos/send_egld.scen.json", &contract_map());
}

#[test]
fn send_esdt() {
	parse_execute_mandos("mandos/send_esdt.scen.json", &contract_map());
}
