use crate::{
    tx_execution::default_execution,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};

use super::{
    change_owner_mock::execute_change_owner, esdt_local_burn::execute_local_burn,
    esdt_local_mint::execute_local_mint, esdt_multi_transfer_mock::execute_esdt_multi_transfer,
    esdt_nft_add_quantity_mock::execute_nft_add_quantity, esdt_nft_burn_mock::execute_nft_burn,
    esdt_nft_create_mock::execute_esdt_nft_create,
    esdt_nft_transfer_mock::execute_esdt_nft_transfer, esdt_transfer_mock::execute_esdt_transfer,
    set_username_mock::execute_set_username, upgrade_contract::execute_upgrade_contract,
};

use elrond_wasm::api::{
    CHANGE_OWNER_BUILTIN_FUNC_NAME, ESDT_LOCAL_BURN_FUNC_NAME, ESDT_LOCAL_MINT_FUNC_NAME,
    ESDT_MULTI_TRANSFER_FUNC_NAME, ESDT_NFT_ADD_QUANTITY_FUNC_NAME, ESDT_NFT_BURN_FUNC_NAME,
    ESDT_NFT_CREATE_FUNC_NAME, ESDT_NFT_TRANSFER_FUNC_NAME, ESDT_TRANSFER_FUNC_NAME,
    SET_USERNAME_FUNC_NAME, UPGRADE_CONTRACT_FUNC_NAME,
};

const ESDT_ROLE_LOCAL_MINT: &[u8] = b"ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &[u8] = b"ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &[u8] = b"ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &[u8] = b"ESDTRoleNFTBurn";

pub fn execute_builtin_function_or_default(
    tx_input: TxInput,
    tx_cache: TxCache,
) -> (TxResult, BlockchainUpdate) {
    match tx_input.func_name.as_slice() {
        ESDT_LOCAL_MINT_FUNC_NAME => check_and_execute_builtin_function(
            ESDT_ROLE_LOCAL_MINT,
            tx_input,
            tx_cache,
            &execute_local_mint,
        ),
        ESDT_LOCAL_BURN_FUNC_NAME => check_and_execute_builtin_function(
            ESDT_ROLE_LOCAL_BURN,
            tx_input,
            tx_cache,
            &execute_local_burn,
        ),
        ESDT_MULTI_TRANSFER_FUNC_NAME => execute_esdt_multi_transfer(tx_input, tx_cache),
        ESDT_NFT_TRANSFER_FUNC_NAME => execute_esdt_nft_transfer(tx_input, tx_cache),
        ESDT_NFT_CREATE_FUNC_NAME => check_and_execute_builtin_function(
            ESDT_ROLE_NFT_CREATE,
            tx_input,
            tx_cache,
            &execute_esdt_nft_create,
        ),
        ESDT_NFT_ADD_QUANTITY_FUNC_NAME => check_and_execute_builtin_function(
            ESDT_ROLE_NFT_ADD_QUANTITY,
            tx_input,
            tx_cache,
            &execute_nft_add_quantity,
        ),
        ESDT_NFT_BURN_FUNC_NAME => check_and_execute_builtin_function(
            ESDT_ROLE_NFT_BURN,
            tx_input,
            tx_cache,
            &execute_nft_burn,
        ),

        ESDT_TRANSFER_FUNC_NAME => execute_esdt_transfer(tx_input, tx_cache),
        CHANGE_OWNER_BUILTIN_FUNC_NAME => execute_change_owner(tx_input, tx_cache),
        SET_USERNAME_FUNC_NAME => execute_set_username(tx_input, tx_cache),
        UPGRADE_CONTRACT_FUNC_NAME => execute_upgrade_contract(tx_input, tx_cache),
        _ => default_execution(tx_input, tx_cache),
    }
}

fn check_and_execute_builtin_function(
    role_name: &[u8],
    tx_input: TxInput,
    tx_cache: TxCache,
    f: &dyn Fn(TxInput, TxCache) -> (TxResult, BlockchainUpdate),
) -> (TxResult, BlockchainUpdate) {
    let check_result = check_allowed_to_execute(role_name, &tx_input, &tx_cache);
    if let Some(tx_result) = check_result {
        return (tx_result, BlockchainUpdate::empty());
    }
    f(tx_input, tx_cache)
}

pub fn check_allowed_to_execute(
    builtin_function_name: &[u8],
    tx_input: &TxInput,
    tx_cache: &TxCache,
) -> Option<TxResult> {
    let token_identifier = tx_input.args[0].clone();
    let available_roles = tx_cache.with_account_mut(&tx_input.to, |account| {
        account.esdt.get_roles(&token_identifier)
    });
    if available_roles.contains(&builtin_function_name.to_vec()) {
        return None;
    }

    Some(TxResult::from_vm_error("action is not allowed".to_string()))
}
