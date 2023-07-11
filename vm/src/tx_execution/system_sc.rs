mod system_sc_issue;
mod system_sc_special_roles;
mod system_sc_unimplemented;

use crate::{
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
    types::VMAddress,
};
use hex_literal::hex;
use system_sc_issue::*;
use system_sc_special_roles::*;
use system_sc_unimplemented::*;

/// Address of the system smart contract that manages ESDT.
/// Bech32: erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u
pub const ESDT_SYSTEM_SC_ADDRESS_ARRAY: [u8; 32] =
    hex!("000000000000000000010000000000000000000000000000000000000002ffff");

pub fn is_system_sc_address(address: &VMAddress) -> bool {
    address.as_array() == &ESDT_SYSTEM_SC_ADDRESS_ARRAY
}

pub fn execute_system_sc(tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
    let func_name = &tx_input.func_name;
    match func_name.as_str() {
        "issue" => issue(tx_input, tx_cache),
        "issueSemiFungible" => issue_semi_fungible(tx_input, tx_cache),
        "issueNonFungible" => issue_non_fungible(tx_input, tx_cache),
        "registerMetaESDT" => register_meta_esdt(tx_input, tx_cache),
        "changeSFTToMetaESDT" => change_sft_to_meta_esdt(tx_input, tx_cache),
        "registerAndSetAllRoles" => register_and_set_all_roles(tx_input, tx_cache),
        "ESDTBurn" => esdt_burn(tx_input, tx_cache),
        "mint" => mint(tx_input, tx_cache),
        "freeze" => freeze(tx_input, tx_cache),
        "unFreeze" => unfreeze(tx_input, tx_cache),
        "wipe" => wipe(tx_input, tx_cache),
        "pause" => pause(tx_input, tx_cache),
        "unPause" => unpause(tx_input, tx_cache),
        "freezeSingleNFT" => freeze_single_nft(tx_input, tx_cache),
        "unFreezeSingleNFT" => unfreeze_single_nft(tx_input, tx_cache),
        "wipeSingleNFT" => wipe_single_nft(tx_input, tx_cache),
        "claim" => claim(tx_input, tx_cache),
        "configChange" => config_change(tx_input, tx_cache),
        "controlChanges" => control_changes(tx_input, tx_cache),
        "transferOwnership" => transfer_ownership(tx_input, tx_cache),
        "getTokenProperties" => get_token_properties(tx_input, tx_cache),
        "getSpecialRoles" => get_special_roles(tx_input, tx_cache),
        "setSpecialRole" => set_special_role(tx_input, tx_cache),
        "unSetSpecialRole" => unset_special_role(tx_input, tx_cache),
        "transferNFTCreateRole" => transfer_nft_create_role(tx_input, tx_cache),
        "stopNFTCreate" => stop_nft_create(tx_input, tx_cache),
        "getAllAddressesAndRoles" => get_all_addresses_and_roles(tx_input, tx_cache),
        "getContractConfig" => get_contract_config(tx_input, tx_cache),
        "changeToMultiShardCreate" => change_to_multi_shard_create(tx_input, tx_cache),
        "setBurnRoleGlobally" => set_burn_role_globally(tx_input, tx_cache),
        "unsetBurnRoleGlobally" => unset_burn_role_globally(tx_input, tx_cache),
        "sendAllTransferRoleAddresses" => send_all_transfer_role_addresses(tx_input, tx_cache),
        invalid_func_name => panic!("invalid system SC function: {invalid_func_name}"),
    }
}
