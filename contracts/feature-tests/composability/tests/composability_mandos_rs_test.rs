use elrond_wasm::*;
use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");

    blockchain.register_contract(
        "file:forwarder/output/forwarder.wasm",
        Box::new(|context| Box::new(forwarder::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:forwarder-raw/output/forwarder-raw.wasm",
        Box::new(|context| Box::new(forwarder_raw::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:proxy-test-first/output/proxy-test-first.wasm",
        Box::new(|context| Box::new(proxy_test_first::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:proxy-test-second/output/proxy-test-second.wasm",
        Box::new(|context| Box::new(proxy_test_second::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:recursive-caller/output/recursive-caller.wasm",
        Box::new(|context| Box::new(recursive_caller::contract_obj(context))),
    );
    blockchain.register_contract(
        "file:vault/output/vault.wasm",
        Box::new(|context| Box::new(vault::contract_obj(context))),
    );
    blockchain
}

#[test]
fn forw_raw_async_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_async_accept_egld.scen.json", world());
}

#[test]
fn forw_raw_async_accept_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_async_accept_esdt.scen.json", world());
}

#[test]
fn forw_raw_async_echo_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_async_echo.scen.json", world());
}

// #[test]
// fn forw_raw_async_send_and_retrieve_multi_transfer_funds_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forw_raw_async_send_and_retrieve_multi_transfer_funds.scen.json",
//         world(),
//     );
// }

#[test]
fn forw_raw_builtin_nft_local_mint_via_async_call_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forw_raw_builtin_nft_local_mint_via_async_call.scen.json",
        world(),
    );
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_sync_call_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forw_raw_builtin_nft_local_mint_via_sync_call.scen.json",
        world(),
    );
}

// #[test]
// fn forw_raw_call_async_retrieve_multi_transfer_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forw_raw_call_async_retrieve_multi_transfer.scen.json",
//         world(),
//     );
// }

#[test]
fn forw_raw_contract_deploy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_contract_deploy.scen.json", world());
}

#[test]
fn forw_raw_contract_upgrade_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_contract_upgrade.scen.json", world());
}

#[test]
fn forw_raw_contract_upgrade_self_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_contract_upgrade_self.scen.json", world());
}

#[test]
fn forw_raw_direct_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_direct_egld.scen.json", world());
}

#[test]
fn forw_raw_direct_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_direct_esdt.scen.json", world());
}

#[test]
fn forw_raw_sync_echo_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_echo.scen.json", world());
}

// #[test]
// fn forw_raw_sync_echo_caller_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_echo_caller.scen.json", world());
// }

#[test]
fn forw_raw_sync_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_egld.scen.json", world());
}

// #[test]
// fn forw_raw_sync_readonly_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_readonly.scen.json", world());
// }

// #[test]
// fn forw_raw_sync_same_context_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_same_context.scen.json", world());
// }

// #[test]
// fn forw_raw_sync_same_context_egld_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forw_raw_sync_same_context_egld.scen.json", world());
// }

#[test]
fn forw_raw_transf_exec_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forw_raw_transf_exec_egld.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_add_quantity_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_builtin_nft_add_quantity.scen.json",
        world(),
    );
}

#[test]
fn forwarder_builtin_nft_burn_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_builtin_nft_burn.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_create_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_builtin_nft_create.scen.json", world());
}

// #[test]
// fn forwarder_builtin_nft_create_by_caller_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forwarder_builtin_nft_create_by_caller.scen.json",
//         world(),
//     );
// }

#[test]
fn forwarder_builtin_nft_local_burn_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_builtin_nft_local_burn.scen.json", world());
}

#[test]
fn forwarder_builtin_nft_local_mint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_builtin_nft_local_mint.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_async_accept_egld.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_async_accept_esdt.scen.json", world());
}

#[test]
fn forwarder_call_async_accept_nft_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_async_accept_nft.scen.json", world());
}

#[test]
fn forwarder_call_async_multi_transfer_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_async_multi_transfer.scen.json",
        world(),
    );
}

// #[test]
// fn forwarder_call_async_retrieve_egld_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forwarder_call_async_retrieve_egld.scen.json",
//         world(),
//     );
// }

// #[test]
// fn forwarder_call_async_retrieve_esdt_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forwarder_call_async_retrieve_esdt.scen.json",
//         world(),
//     );
// }

// #[test]
// fn forwarder_call_async_retrieve_nft_rs() {
//     elrond_wasm_debug::mandos_rs(
//         "mandos/forwarder_call_async_retrieve_nft.scen.json",
//         world(),
//     );
// }

#[test]
fn forwarder_call_sync_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_sync_accept_egld.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_sync_accept_esdt.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_accept_multi_transfer.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_nft_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_sync_accept_nft.scen.json", world());
}

#[test]
fn forwarder_call_sync_accept_then_read_egld_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_accept_then_read_egld.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_then_read_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_accept_then_read_esdt.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_accept_then_read_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_egld_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_retrieve_egld.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_sync_retrieve_esdt.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_sync_retrieve_nft_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_call_sync_retrieve_nft.scen.json", world());
}

#[test]
fn forwarder_call_transf_exec_accept_egld_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_egld.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_egld_twice_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_egld_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_esdt.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_twice_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_esdt_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_nft_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_nft.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_return_values.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_accept_sft_twice_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_accept_sft_twice.scen.json",
        world(),
    );
}

#[test]
fn forwarder_call_transf_exec_multi_transfer_esdt_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_call_transf_exec_multi_transfer_esdt.scen.json",
        world(),
    );
}

#[test]
fn forwarder_contract_change_owner_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_contract_change_owner.scen.json", world());
}

#[test]
fn forwarder_contract_deploy_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_contract_deploy.scen.json", world());
}

#[test]
fn forwarder_contract_upgrade_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_contract_upgrade.scen.json", world());
}

#[test]
fn forwarder_nft_create_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_create.scen.json", world());
}

#[test]
fn forwarder_nft_create_and_send_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_create_and_send.scen.json", world());
}

#[test]
fn forwarder_nft_current_nonce_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_current_nonce.scen.json", world());
}

#[test]
fn forwarder_nft_decode_complex_attributes_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_nft_decode_complex_attributes.scen.json",
        world(),
    );
}

#[test]
fn forwarder_nft_transfer_async_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_transfer_async.scen.json", world());
}

#[test]
fn forwarder_nft_transfer_exec_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_nft_transfer_exec.scen.json", world());
}

#[test]
fn forwarder_no_endpoint_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_no_endpoint.scen.json", world());
}

#[test]
fn forwarder_send_esdt_multi_transfer_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/forwarder_send_esdt_multi_transfer.scen.json",
        world(),
    );
}

// #[test]
// fn forwarder_send_twice_egld_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forwarder_send_twice_egld.scen.json", world());
// }

// #[test]
// fn forwarder_send_twice_esdt_rs() {
//     elrond_wasm_debug::mandos_rs("mandos/forwarder_send_twice_esdt.scen.json", world());
// }

#[test]
fn forwarder_sync_echo_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_sync_echo.scen.json", world());
}

#[test]
fn forwarder_sync_echo_range_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_sync_echo_range.scen.json", world());
}

#[test]
fn forwarder_tranfer_esdt_with_fees_rs() {
    elrond_wasm_debug::mandos_rs("mandos/forwarder_tranfer_esdt_with_fees.scen.json", world());
}

#[test]
fn proxy_test_init_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_init.scen.json", world());
}

#[test]
fn proxy_test_message_othershard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_message_otherShard.scen.json", world());
}

#[test]
fn proxy_test_message_othershard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_message_otherShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_message_sameshard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_message_sameShard.scen.json", world());
}

#[test]
fn proxy_test_message_sameshard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_message_sameShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_payment_othershard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_payment_otherShard.scen.json", world());
}

#[test]
fn proxy_test_payment_othershard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_otherShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_payment_sameshard_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_payment_sameShard.scen.json", world());
}

#[test]
fn proxy_test_payment_sameshard_callback_rs() {
    elrond_wasm_debug::mandos_rs(
        "mandos/proxy_test_payment_sameShard_callback.scen.json",
        world(),
    );
}

#[test]
fn proxy_test_upgrade_rs() {
    elrond_wasm_debug::mandos_rs("mandos/proxy_test_upgrade.scen.json", world());
}

#[test]
fn recursive_caller_egld_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/recursive_caller_egld_1.scen.json", world());
}

#[test]
fn recursive_caller_esdt_1_rs() {
    elrond_wasm_debug::mandos_rs("mandos/recursive_caller_esdt_1.scen.json", world());
}

#[test]
fn send_egld_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_egld.scen.json", world());
}

#[test]
fn send_esdt_rs() {
    elrond_wasm_debug::mandos_rs("mandos/send_esdt.scen.json", world());
}
