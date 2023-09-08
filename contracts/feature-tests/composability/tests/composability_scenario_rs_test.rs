use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");

    blockchain.register_contract(
        "file:builtin-func-features/output/builtin-func-features.wasm",
        builtin_func_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:forwarder-queue/output/forwarder-queue.wasm",
        forwarder_queue::ContractBuilder,
    );
    blockchain.register_contract(
        "file:forwarder/output/forwarder.wasm",
        forwarder::ContractBuilder,
    );
    blockchain.register_contract(
        "file:forwarder-raw/output/forwarder-raw.wasm",
        forwarder_raw::ContractBuilder,
    );
    blockchain.register_contract(
        "file:promises-features/output/promises-features.wasm",
        promises_features::ContractBuilder,
    );
    blockchain.register_contract(
        "file:proxy-test-first/output/proxy-test-first.wasm",
        proxy_test_first::ContractBuilder,
    );
    blockchain.register_contract(
        "file:proxy-test-second/output/proxy-test-second.wasm",
        proxy_test_second::ContractBuilder,
    );
    blockchain.register_contract(
        "file:recursive-caller/output/recursive-caller.wasm",
        recursive_caller::ContractBuilder,
    );
    blockchain.register_contract("file:vault/output/vault.wasm", vault::ContractBuilder);
    blockchain
}

#[test]
#[ignore = "not yet supported"]
fn promises_multi_transfer_rs() {
    world().run("scenarios-promises/promises_multi_transfer.scen.json");
}

#[test]
#[ignore = "not yet supported"]
fn promises_single_transfer_rs() {
    world().run("scenarios-promises/promises_single_transfer.scen.json");
}

#[test]
fn builtin_func_delete_user_name_rs() {
    world().run("scenarios/builtin_func_delete_user_name.scen.json");
}

#[test]
fn builtin_func_set_user_name_rs() {
    world().run("scenarios/builtin_func_set_user_name.scen.json");
}

#[test]
fn forw_queue_async_rs() {
    world().run("scenarios/forw_queue_async.scen.json");
}

#[test]
fn forw_raw_async_accept_egld_rs() {
    world().run("scenarios/forw_raw_async_accept_egld.scen.json");
}

#[test]
fn forw_raw_async_accept_esdt_rs() {
    world().run("scenarios/forw_raw_async_accept_esdt.scen.json");
}

#[test]
fn forw_raw_async_echo_rs() {
    world().run("scenarios/forw_raw_async_echo.scen.json");
}

#[test]
fn forw_raw_async_send_and_retrieve_multi_transfer_funds_rs() {
    world().run("scenarios/forw_raw_async_send_and_retrieve_multi_transfer_funds.scen.json");
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_async_call_rs() {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_async_call.scen.json");
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_sync_call_rs() {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_sync_call.scen.json");
}

#[test]
fn forw_raw_call_async_retrieve_multi_transfer_rs() {
    world().run("scenarios/forw_raw_call_async_retrieve_multi_transfer.scen.json");
}

#[test]
fn forw_raw_contract_deploy_rs() {
    world().run("scenarios/forw_raw_contract_deploy.scen.json");
}

#[test]
fn forw_raw_contract_upgrade_rs() {
    world().run("scenarios/forw_raw_contract_upgrade.scen.json");
}

#[test]
fn forw_raw_contract_upgrade_self_rs() {
    world().run("scenarios/forw_raw_contract_upgrade_self.scen.json");
}

#[test]
fn forw_raw_direct_egld_rs() {
    world().run("scenarios/forw_raw_direct_egld.scen.json");
}

#[test]
fn forw_raw_direct_esdt_rs() {
    world().run("scenarios/forw_raw_direct_esdt.scen.json");
}

#[test]
fn forw_raw_direct_multi_esdt_rs() {
    world().run("scenarios/forw_raw_direct_multi_esdt.scen.json");
}

#[test]
#[ignore = "not yet supported"]
fn forw_raw_init_async_rs() {
    world().run("scenarios/forw_raw_init_async.scen.json");
}

#[test]
#[ignore = "not yet supported"]
fn forw_raw_init_sync_accept_egld_rs() {
    world().run("scenarios/forw_raw_init_sync_accept_egld.scen.json");
}

#[test]
#[ignore = "not yet supported"]
fn forw_raw_init_sync_echo_rs() {
    world().run("scenarios/forw_raw_init_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_rs() {
    world().run("scenarios/forw_raw_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_caller_rs() {
    world().run("scenarios/forw_raw_sync_echo_caller.scen.json");
}

#[test]
fn forw_raw_sync_egld_rs() {
    world().run("scenarios/forw_raw_sync_egld.scen.json");
}

#[test]
#[ignore]
fn forw_raw_sync_readonly_rs() {
    world().run("scenarios/forw_raw_sync_readonly.scen.json");
}

#[test]
#[ignore]
fn forw_raw_sync_same_context_rs() {
    world().run("scenarios/forw_raw_sync_same_context.scen.json");
}

#[test]
#[ignore]
fn forw_raw_sync_same_context_egld_rs() {
    world().run("scenarios/forw_raw_sync_same_context_egld.scen.json");
}

#[test]
fn forw_raw_transf_exec_accept_egld_rs() {
    world().run("scenarios/forw_raw_transf_exec_accept_egld.scen.json");
}

#[test]
fn forw_raw_transf_exec_reject_egld_rs() {
    world().run("scenarios/forw_raw_transf_exec_reject_egld.scen.json");
}

#[test]
fn forwarder_builtin_nft_add_quantity_rs() {
    world().run("scenarios/forwarder_builtin_nft_add_quantity.scen.json");
}

#[test]
fn forwarder_builtin_nft_burn_rs() {
    world().run("scenarios/forwarder_builtin_nft_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_create_rs() {
    world().run("scenarios/forwarder_builtin_nft_create.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_burn_rs() {
    world().run("scenarios/forwarder_builtin_nft_local_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_mint_rs() {
    world().run("scenarios/forwarder_builtin_nft_local_mint.scen.json");
}

#[test]
fn forwarder_call_async_accept_egld_rs() {
    world().run("scenarios/forwarder_call_async_accept_egld.scen.json");
}

#[test]
fn forwarder_call_async_accept_esdt_rs() {
    world().run("scenarios/forwarder_call_async_accept_esdt.scen.json");
}

#[test]
fn forwarder_call_async_accept_nft_rs() {
    world().run("scenarios/forwarder_call_async_accept_nft.scen.json");
}

#[test]
fn forwarder_call_async_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_async_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_egld_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_egld.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_esdt_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_esdt.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_nft_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_nft.scen.json");
}

#[test]
fn forwarder_call_sync_accept_egld_rs() {
    world().run("scenarios/forwarder_call_sync_accept_egld.scen.json");
}

#[test]
fn forwarder_call_sync_accept_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_accept_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_sync_accept_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_sync_accept_nft_rs() {
    world().run("scenarios/forwarder_call_sync_accept_nft.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_egld_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_egld.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_nft.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_egld_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_egld.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_nft_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_nft.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_egld_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_egld.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_egld_twice_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_egld_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_esdt.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_esdt_twice_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_esdt_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_nft_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_nft.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_return_values.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_sft_twice_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_sft_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_reject_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_transf_exec_reject_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_transf_exec_reject_nft_rs() {
    world().run("scenarios/forwarder_call_transf_exec_reject_nft.scen.json");
}

#[test]
fn forwarder_contract_change_owner_rs() {
    world().run("scenarios/forwarder_contract_change_owner.scen.json");
}

#[test]
fn forwarder_contract_deploy_rs() {
    world().run("scenarios/forwarder_contract_deploy.scen.json");
}

#[test]
fn forwarder_contract_upgrade_rs() {
    world().run("scenarios/forwarder_contract_upgrade.scen.json");
}

#[test]
fn forwarder_get_esdt_local_roles_rs() {
    world().run("scenarios/forwarder_get_esdt_local_roles.scen.json");
}

#[test]
fn forwarder_get_esdt_token_data_rs() {
    world().run("scenarios/forwarder_get_esdt_token_data.scen.json");
}

#[test]
fn forwarder_nft_add_uri_rs() {
    world().run("scenarios/forwarder_nft_add_uri.scen.json");
}

#[test]
fn forwarder_nft_create_rs() {
    world().run("scenarios/forwarder_nft_create.scen.json");
}

#[test]
fn forwarder_nft_create_and_send_rs() {
    world().run("scenarios/forwarder_nft_create_and_send.scen.json");
}

#[test]
fn forwarder_nft_current_nonce_rs() {
    world().run("scenarios/forwarder_nft_current_nonce.scen.json");
}

#[test]
fn forwarder_nft_decode_complex_attributes_rs() {
    world().run("scenarios/forwarder_nft_decode_complex_attributes.scen.json");
}

#[test]
fn forwarder_nft_transfer_async_rs() {
    world().run("scenarios/forwarder_nft_transfer_async.scen.json");
}

#[test]
fn forwarder_nft_transfer_exec_rs() {
    world().run("scenarios/forwarder_nft_transfer_exec.scen.json");
}

#[test]
fn forwarder_nft_update_attributes_rs() {
    world().run("scenarios/forwarder_nft_update_attributes.scen.json");
}

#[test]
fn forwarder_no_endpoint_rs() {
    world().run("scenarios/forwarder_no_endpoint.scen.json");
}

#[test]
fn forwarder_retrieve_funds_with_accept_func_rs() {
    world().run("scenarios/forwarder_retrieve_funds_with_accept_func.scen.json");
}

#[test]
fn forwarder_send_esdt_multi_transfer_rs() {
    world().run("scenarios/forwarder_send_esdt_multi_transfer.scen.json");
}

#[test]
#[ignore]
fn forwarder_send_twice_egld_rs() {
    world().run("scenarios/forwarder_send_twice_egld.scen.json");
}

#[test]
#[ignore]
fn forwarder_send_twice_esdt_rs() {
    world().run("scenarios/forwarder_send_twice_esdt.scen.json");
}

#[test]
fn forwarder_sync_echo_rs() {
    world().run("scenarios/forwarder_sync_echo.scen.json");
}

#[test]
fn forwarder_tranfer_esdt_with_fees_rs() {
    world().run("scenarios/forwarder_tranfer_esdt_with_fees.scen.json");
}

#[test]
fn forwarder_validate_token_identifier_rs() {
    world().run("scenarios/forwarder_validate_token_identifier.scen.json");
}

#[test]
fn proxy_test_init_rs() {
    world().run("scenarios/proxy_test_init.scen.json");
}

#[test]
fn proxy_test_message_other_shard_rs() {
    world().run("scenarios/proxy_test_message_otherShard.scen.json");
}

#[test]
fn proxy_test_message_other_shard_callback_rs() {
    world().run("scenarios/proxy_test_message_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_message_same_shard_rs() {
    world().run("scenarios/proxy_test_message_sameShard.scen.json");
}

#[test]
fn proxy_test_message_same_shard_callback_rs() {
    world().run("scenarios/proxy_test_message_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_rs() {
    world().run("scenarios/proxy_test_payment_otherShard.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_callback_rs() {
    world().run("scenarios/proxy_test_payment_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_rs() {
    world().run("scenarios/proxy_test_payment_sameShard.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_callback_rs() {
    world().run("scenarios/proxy_test_payment_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_upgrade_rs() {
    world().run("scenarios/proxy_test_upgrade.scen.json");
}

#[test]
fn recursive_caller_egld_1_rs() {
    world().run("scenarios/recursive_caller_egld_1.scen.json");
}

#[test]
fn recursive_caller_esdt_1_rs() {
    world().run("scenarios/recursive_caller_esdt_1.scen.json");
}

#[test]
fn send_egld_rs() {
    world().run("scenarios/send_egld.scen.json");
}

#[test]
fn send_esdt_rs() {
    world().run("scenarios/send_esdt.scen.json");
}
