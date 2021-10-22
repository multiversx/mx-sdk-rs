use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    change_owner_mock::execute_change_owner, esdt_multi_transfer_mock::execute_esdt_multi_transfer,
    esdt_nft_add_quantity_mock::execute_nft_add_quantity,
    esdt_nft_create_mock::execute_esdt_nft_create,
    esdt_nft_transfer_mock::execute_esdt_nft_transfer, esdt_transfer_mock::execute_esdt_transfer,
    set_username_mock::execute_set_username, upgrade_contract::execute_upgrade_contract,
};

use elrond_wasm::api::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_ADD_QUANTITY_FUNC_NAME,
    ESDT_NFT_CREATE_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME,
    SET_USERNAME_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
};

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    match tx_input.func_name.as_slice() {
        ESDT_TRANSFER_FUNC_NAME => execute_esdt_transfer(tx_input, tx_cache),
        ESDT_NFT_TRANSFER_FUNC_NAME => execute_esdt_nft_transfer(tx_input, tx_cache),
        ESDT_MULTI_TRANSFER_FUNC_NAME => execute_esdt_multi_transfer(tx_input, tx_cache),
        ESDT_NFT_CREATE_FUNC_NAME => execute_esdt_nft_create(tx_input, tx_cache),
        ESDT_NFT_ADD_QUANTITY_FUNC_NAME => execute_nft_add_quantity(tx_input, tx_cache),
        CHANGE_OWNER_BUILTIN_FUNC_NAME => execute_change_owner(tx_input, tx_cache),
        SET_USERNAME_FUNC_NAME => execute_set_username(tx_input, tx_cache),
        UPGRADE_CONTRACT_FUNC_NAME => execute_upgrade_contract(tx_input, tx_cache),
        _ => default_execution(tx_input, tx_cache),
    }
}
