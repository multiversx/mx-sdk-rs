use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();

    contract_map.register_contract(
        "file:../forwarder/output/forwarder.wasm",
        Box::new(|context| Box::new(forwarder::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../forwarder-raw/output/forwarder-raw.wasm",
        Box::new(|context| Box::new(forwarder_raw::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../proxy-test-first/output/proxy-test-first.wasm",
        Box::new(|context| Box::new(proxy_test_first::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../proxy-test-second/output/proxy-test-second.wasm",
        Box::new(|context| Box::new(proxy_test_second::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../vault/output/vault.wasm",
        Box::new(|context| Box::new(vault::contract_obj(context))),
    );

    contract_map
}
#[test]
fn forw_raw_async_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forw_raw_async_accept_egld.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn forw_raw_async_accept_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forw_raw_async_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn forw_raw_async_echo_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_async_echo.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_direct_egld.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_direct_esdt.scen.json", &contract_map());
}

// #[test]
// fn forw_raw_sync_echo_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forw_raw_sync_egld_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_egld.scen.json", &contract_map());
// }

#[test]
fn forwarder_call_async_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_async_accept_egld.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn forwarder_call_async_accept_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_async_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_accept_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_async_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_async_retrieve_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_async_retrieve_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_async_retrieve_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_then_read_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_then_read_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_accept_then_read_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_retrieve_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_retrieve_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_sync_retrieve_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_egld_twice_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_egld_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_esdt_twice_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_esdt_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_nft_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_return_values_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_return_values.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_sft_twice_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_call_transf_exec_accept_sft_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_change_owner_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_contract_change_owner.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_deploy_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_contract_deploy.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_upgrade_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_contract_upgrade.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_nft_create_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_create.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_async_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_nft_transfer_async.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_nft_transfer_exec_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_nft_transfer_exec.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_send_twice_egld_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_send_twice_egld.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_send_twice_esdt_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_send_twice_esdt.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_echo_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/forwarder_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo_range_rs() {
// 	elrond_wasm_debug::mandos_rs(
// 		"mandos/forwarder_sync_echo_range.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn proxy_test_message_othershard_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_message_otherShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_othershard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_message_otherShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_sameshard_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_message_sameShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_sameshard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "/home/andreim/elrond/smartcontract/elrond-wasm-rs/contracts/feature-tests/composability/mandos/proxy_test_message_sameShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_othershard_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_otherShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_othershard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_otherShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_sameshard_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_sameShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_sameshard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_sameShard_callback.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn recursive_caller_egld_1_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_egld_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_egld_2_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_egld_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_egld_x_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_egld_x.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_1_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_esdt_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_2_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_esdt_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_esdt_x_rs() {
// 	elrond_wasm_debug::mandos_rs("mandos/recursive_caller_esdt_x.scen.json", &contract_map());
// }

#[test]
fn send_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_egld.scen.json", &contract_map());
}

#[test]
fn send_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_esdt.scen.json", &contract_map());
}
