use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/composability");

    blockchain.register_contract(
        "mxsc:forwarder/output/forwarder.mxsc.json",
        forwarder_legacy::ContractBuilder,
    );

    let vault_sc_config =
        meta::multi_contract_config::<vault::AbiProvider>(&blockchain.current_dir().join("vault"));
    blockchain.register_contract_variant(
        "mxsc:vault/output/vault.mxsc.json",
        vault::ContractBuilder,
        vault_sc_config.find_contract("vault"),
    );
    blockchain.register_contract_variant(
        "mxsc:vault/output/vault-upgrade.mxsc.json",
        vault::ContractBuilder,
        vault_sc_config.find_contract("vault-upgrade"),
    );
    blockchain
}

#[test]
fn legacy_forwarder_builtin_nft_add_quantity_rs() {
    world().run("scenarios/forwarder_builtin_nft_add_quantity.scen.json");
}

#[test]
fn legacy_forwarder_builtin_nft_burn_rs() {
    world().run("scenarios/forwarder_builtin_nft_burn.scen.json");
}

#[test]
fn legacy_forwarder_builtin_nft_create_rs() {
    world().run("scenarios/forwarder_builtin_nft_create.scen.json");
}

#[test]
fn legacy_forwarder_builtin_nft_local_burn_rs() {
    world().run("scenarios/forwarder_builtin_nft_local_burn.scen.json");
}

#[test]
fn legacy_forwarder_builtin_nft_local_mint_rs() {
    world().run("scenarios/forwarder_builtin_nft_local_mint.scen.json");
}

#[test]
fn legacy_forwarder_call_async_accept_egld_rs() {
    world().run("scenarios/forwarder_call_async_accept_egld.scen.json");
}

#[test]
fn legacy_forwarder_call_async_accept_esdt_rs() {
    world().run("scenarios/forwarder_call_async_accept_esdt.scen.json");
}

#[test]
fn legacy_forwarder_call_async_accept_nft_rs() {
    world().run("scenarios/forwarder_call_async_accept_nft.scen.json");
}

#[test]
fn legacy_forwarder_call_async_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_async_multi_transfer.scen.json");
}

#[test]
#[ignore = "no longer matching new implementation"]
fn legacy_forwarder_call_async_retrieve_egld_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_egld.scen.json");
}

#[test]
#[ignore = "no longer matching new implementation"]
fn legacy_forwarder_call_async_retrieve_esdt_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_esdt.scen.json");
}

#[test]
#[ignore = "no longer matching new implementation"]
fn legacy_forwarder_call_async_retrieve_nft_rs() {
    world().run("scenarios/forwarder_call_async_retrieve_nft.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_egld_rs() {
    world().run("scenarios/forwarder_call_sync_accept_egld.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_accept_esdt.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_multi_transfer_rs() {
    world().run("scenarios/forwarder_call_sync_accept_multi_transfer.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_nft_rs() {
    world().run("scenarios/forwarder_call_sync_accept_nft.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_then_read_egld_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_egld.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_then_read_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_esdt.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_accept_then_read_nft_rs() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_nft.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_retrieve_egld_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_egld.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_retrieve_esdt_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_esdt.scen.json");
}

#[test]
fn legacy_forwarder_call_sync_retrieve_nft_rs() {
    world().run("scenarios/forwarder_call_sync_retrieve_nft.scen.json");
}

#[test]
#[ignore = "TODO: fix logs"]
fn legacy_forwarder_call_transf_exec_accept_return_values_rs() {
    world().run("scenarios/forwarder_call_transf_exec_accept_return_values.scen.json");
}

#[test]
#[ignore = "TODO: fix logs"]
fn legacy_forwarder_call_transf_exec_egld_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_egld_accept.scen.json");
}

#[test]
#[ignore = "TODO: fix logs"]
fn legacy_forwarder_call_transf_exec_egld_accept_twice_rs() {
    world().run("scenarios/forwarder_call_transf_exec_egld_accept_twice.scen.json");
}

#[test]
#[ignore = "TODO: fix logs"]
fn legacy_forwarder_call_transf_exec_multi_transfer_egld_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_egld_accept.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_multi_transfer_egld_reject_rs() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_egld_reject.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_multi_transfer_esdt_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_esdt_accept.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_multi_transfer_esdt_reject_rs() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_esdt_reject.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_single_esdt_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_single_esdt_accept.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_single_esdt_accept_twice_rs() {
    world().run("scenarios/forwarder_call_transf_exec_single_esdt_accept_twice.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_single_nft_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_single_nft_accept.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_single_nft_reject_rs() {
    world().run("scenarios/forwarder_call_transf_exec_single_nft_reject.scen.json");
}

#[test]
fn legacy_forwarder_call_transf_exec_single_sft_twice_accept_rs() {
    world().run("scenarios/forwarder_call_transf_exec_single_sft_twice_accept.scen.json");
}

#[test]
fn legacy_forwarder_contract_change_owner_rs() {
    world().run("scenarios/forwarder_contract_change_owner.scen.json");
}

#[test]
fn legacy_forwarder_contract_deploy_rs() {
    world().run("scenarios/forwarder_contract_deploy.scen.json");
}

#[test]
fn legacy_forwarder_contract_upgrade_rs() {
    world().run("scenarios/forwarder_contract_upgrade.scen.json");
}

#[test]
fn legacy_forwarder_nft_add_uri_rs() {
    world().run("scenarios/forwarder_nft_add_uri.scen.json");
}

#[test]
fn legacy_forwarder_nft_create_rs() {
    world().run("scenarios/forwarder_nft_create.scen.json");
}

#[test]
fn legacy_forwarder_nft_create_and_send_rs() {
    world().run("scenarios/forwarder_nft_create_and_send.scen.json");
}

#[test]
fn legacy_forwarder_nft_decode_complex_attributes_rs() {
    world().run("scenarios/forwarder_nft_decode_complex_attributes.scen.json");
}

#[test]
fn legacy_forwarder_nft_transfer_async_rs() {
    world().run("scenarios/forwarder_nft_transfer_async.scen.json");
}

#[test]
fn legacy_forwarder_nft_transfer_exec_rs() {
    world().run("scenarios/forwarder_nft_transfer_exec.scen.json");
}

#[test]
fn legacy_forwarder_nft_update_attributes_rs() {
    world().run("scenarios/forwarder_nft_update_attributes.scen.json");
}

#[test]
fn legacy_forwarder_no_endpoint_rs() {
    world().run("scenarios/forwarder_no_endpoint.scen.json");
}

#[test]
fn legacy_forwarder_retrieve_funds_with_accept_func_rs() {
    world().run("scenarios/forwarder_retrieve_funds_with_accept_func.scen.json");
}

#[test]
fn legacy_forwarder_send_esdt_multi_transfer_rs() {
    world().run("scenarios/forwarder_send_esdt_multi_transfer.scen.json");
}

#[test]
fn legacy_forwarder_sync_echo_rs() {
    world().run("scenarios/forwarder_sync_echo.scen.json");
}

#[test]
fn legacy_forwarder_transfer_esdt_with_fees_rs() {
    world().run("scenarios/forwarder_transfer_esdt_with_fees.scen.json");
}

#[test]
fn legacy_send_egld_rs() {
    world().run("scenarios/send_egld.scen.json");
}

#[test]
fn legacy_send_esdt_rs() {
    world().run("scenarios/send_esdt.scen.json");
}
