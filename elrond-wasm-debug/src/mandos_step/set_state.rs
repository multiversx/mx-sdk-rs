use std::collections::BTreeMap;

use mandos::{Account, AddressKey, BlockInfo, NewAddress};

use crate::{
    account_esdt::EsdtData, esdt_instance::EsdtInstance, AccountData, BlockInfo as CrateBlockInfo,
    BlockchainMock,
};

pub fn execute(
    state: &mut BlockchainMock,
    accounts: &BTreeMap<AddressKey, Account>,
    new_addresses: &[NewAddress],
    previous_block_info: &Option<BlockInfo>,
    current_block_info: &Option<BlockInfo>,
) {
    for (address, account) in accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let esdt = account
            .esdt
            .iter()
            .map(|(k, v)| {
                (
                    k.value.clone(),
                    EsdtData {
                        token_identifier: v
                            .token_identifier
                            .as_ref()
                            .map(|token_identifier| token_identifier.value.clone())
                            .unwrap_or_default(),
                        instances: v
                            .instances
                            .iter()
                            .map(|(k1, v1)| {
                                (
                                    k1.value.clone(),
                                    EsdtInstance {
                                        value: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                        esdt_type: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                        name: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                        creator: v1
                                            .creator
                                            .as_ref()
                                            .map(|creator| creator.value.clone())
                                            .unwrap_or_default(),
                                        reserved: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                        royalties: v1
                                            .royalties
                                            .as_ref()
                                            .map(|royalties| royalties.value.clone())
                                            .unwrap_or_default(),
                                        hash: v1
                                            .hash
                                            .as_ref()
                                            .map(|hash| hash.value.clone())
                                            .unwrap_or_default(),
                                        uri: v1
                                            .uri
                                            .as_ref()
                                            .map(|uri| uri.value.clone())
                                            .unwrap_or_default(),
                                        properties: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                        attributes: v1
                                            .balance
                                            .as_ref()
                                            .map(|value| value.value.clone())
                                            .unwrap_or_default(),
                                    },
                                )
                            })
                            .collect(),
                        last_nonce: v
                            .last_nonce
                            .as_ref()
                            .map(|last_nonce| last_nonce.value.clone())
                            .unwrap_or_default(),
                        roles: v
                            .roles
                            .as_ref()
                            .map(|roles| roles.value.clone())
                            .unwrap_or_default(),
                        frozen: v
                            .frozen
                            .as_ref()
                            .map(|frozen| frozen.value.clone())
                            .unwrap_or_default(),
                    },
                )
            })
            .collect();

        state.validate_and_add_account(AccountData {
            address: address.value.into(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            balance: account
                .balance
                .as_ref()
                .map(|balance| balance.value.clone())
                .unwrap_or_default(),
            esdt,
            username: account
                .username
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone())
                .unwrap_or_default(),
            storage,
            contract_path: account
                .code
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone()),
            contract_owner: account
                .owner
                .as_ref()
                .map(|address_value| address_value.value.into()),
        });
    }
    for new_address in new_addresses.iter() {
        assert!(
            state.is_smart_contract_address(&new_address.new_address.value.into()),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.value.into(),
            new_address.creator_nonce.value,
            new_address.new_address.value.into(),
        )
    }
    if let Some(block_info_obj) = &*previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn update_block_info(block_info: &mut CrateBlockInfo, mandos_block_info: &mandos::BlockInfo) {
    if let Some(u64_value) = &mandos_block_info.block_timestamp {
        block_info.block_timestamp = u64_value.value;
    }
    if let Some(u64_value) = &mandos_block_info.block_nonce {
        block_info.block_nonce = u64_value.value;
    }
    if let Some(u64_value) = &mandos_block_info.block_epoch {
        block_info.block_epoch = u64_value.value;
    }
    if let Some(u64_value) = &mandos_block_info.block_round {
        block_info.block_round = u64_value.value;
    }
    if let Some(bytes_value) = &mandos_block_info.block_random_seed {
        const SEED_LEN: usize = 48;
        let val = &bytes_value.value;

        assert!(
            val.len() == SEED_LEN,
            "block random seed input value must be exactly 48 bytes long"
        );

        let mut seed = [0u8; SEED_LEN];
        seed[..].copy_from_slice(val.as_slice());
        block_info.block_random_seed = Box::from(seed);
    }
}
