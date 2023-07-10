mod system_sc_issue;
mod system_sc_special_roles;
mod system_sc_unimplemented;

use crate::{
    tx_mock::{TxContext, TxResult},
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

pub fn execute_system_sc(tx_context: TxContext) -> (TxContext, TxResult) {
    let func_name = tx_context.tx_input_box.func_name.clone();
    match func_name.as_str() {
        "issue" => issue(tx_context),
        "issueSemiFungible" => issue_semi_fungible(tx_context),
        "issueNonFungible" => issue_non_fungible(tx_context),
        "registerMetaESDT" => register_meta_esdt(tx_context),
        "changeSFTToMetaESDT" => change_sft_to_meta_esdt(tx_context),
        "registerAndSetAllRoles" => register_and_set_all_roles(tx_context),
        "ESDTBurn" => esdt_burn(tx_context),
        "mint" => mint(tx_context),
        "freeze" => freeze(tx_context),
        "unFreeze" => unfreeze(tx_context),
        "wipe" => wipe(tx_context),
        "pause" => pause(tx_context),
        "unPause" => unpause(tx_context),
        "freezeSingleNFT" => freeze_single_nft(tx_context),
        "unFreezeSingleNFT" => unfreeze_single_nft(tx_context),
        "wipeSingleNFT" => wipe_single_nft(tx_context),
        "claim" => claim(tx_context),
        "configChange" => config_change(tx_context),
        "controlChanges" => control_changes(tx_context),
        "transferOwnership" => transfer_ownership(tx_context),
        "getTokenProperties" => get_token_properties(tx_context),
        "getSpecialRoles" => get_special_roles(tx_context),
        "setSpecialRole" => set_special_role(tx_context),
        "unSetSpecialRole" => unset_special_role(tx_context),
        "transferNFTCreateRole" => transfer_nft_create_role(tx_context),
        "stopNFTCreate" => stop_nft_create(tx_context),
        "getAllAddressesAndRoles" => get_all_addresses_and_roles(tx_context),
        "getContractConfig" => get_contract_config(tx_context),
        "changeToMultiShardCreate" => change_to_multi_shard_create(tx_context),
        "setBurnRoleGlobally" => set_burn_role_globally(tx_context),
        "unsetBurnRoleGlobally" => unset_burn_role_globally(tx_context),
        "sendAllTransferRoleAddresses" => send_all_transfer_role_addresses(tx_context),
        invalid_func_name => panic!("invalid system SC function: {invalid_func_name}"),
    }
}
