use std::collections::BTreeMap;

use elrond_wasm::types::Address;
use mandos::model::{Account, AddressKey, BlockInfo, NewAddress};
use num_bigint::BigUint;

use crate::world_mock::{
    is_smart_contract_address, AccountData, AccountEsdt, BlockInfo as CrateBlockInfo,
    BlockchainMock, EsdtData, EsdtInstance, EsdtInstanceMetadata, EsdtInstances, EsdtRoles,
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
        let esdt = AccountEsdt::new_from_raw_map(
            account
                .esdt
                .iter()
                .map(|(k, v)| {
                    (
                        k.value.clone(),
                        convert_mandos_esdt_to_world_mock(k.value.as_slice(), v),
                    )
                })
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.value.into(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            egld_balance: account
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
            is_smart_contract_address(&new_address.new_address.value.into()),
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

fn convert_mandos_esdt_to_world_mock(
    token_identifier: &[u8],
    mandos_esdt: &mandos::model::Esdt,
) -> EsdtData {
    match mandos_esdt {
        mandos::model::Esdt::Short(short_esdt) => {
            let balance = BigUint::from_bytes_be(short_esdt.value.as_slice());
            let mut esdt_data = EsdtData {
                token_identifier: token_identifier.to_vec(),
                ..Default::default()
            };
            esdt_data.instances.add(0, balance);
            esdt_data
        },
        mandos::model::Esdt::Full(full_esdt) => EsdtData {
            token_identifier: full_esdt
                .token_identifier
                .as_ref()
                .map(|token_identifier| token_identifier.value.clone())
                .unwrap_or_default(),
            instances: EsdtInstances::new_from_hash(
                full_esdt
                    .instances
                    .iter()
                    .map(|mandos_instance| {
                        let mock_instance =
                            convert_mandos_esdt_instance_to_world_mock(mandos_instance);
                        (mock_instance.nonce, mock_instance)
                    })
                    .collect(),
            ),
            last_nonce: full_esdt
                .last_nonce
                .as_ref()
                .map(|last_nonce| last_nonce.value)
                .unwrap_or_default(),
            roles: EsdtRoles::new(
                full_esdt
                    .roles
                    .iter()
                    .map(|role| role.value.clone())
                    .collect(),
            ),
            frozen: if let Some(u64_value) = &full_esdt.frozen {
                u64_value.value > 0
            } else {
                false
            },
        },
    }
}

fn convert_mandos_esdt_instance_to_world_mock(
    mandos_esdt: &mandos::model::Instance,
) -> EsdtInstance {
    EsdtInstance {
        nonce: mandos_esdt
            .nonce
            .as_ref()
            .map(|nonce| nonce.value)
            .unwrap_or_default(),
        balance: mandos_esdt
            .balance
            .as_ref()
            .map(|value| value.value.clone())
            .unwrap_or_default(),
        metadata: EsdtInstanceMetadata {
            name: Vec::new(),
            creator: mandos_esdt
                .creator
                .as_ref()
                .map(|creator| Address::from_slice(creator.value.as_slice())),
            royalties: mandos_esdt
                .royalties
                .as_ref()
                .map(|royalties| royalties.value)
                .unwrap_or_default(),
            hash: mandos_esdt.hash.as_ref().map(|hash| hash.value.clone()),
            uri: mandos_esdt.uri.as_ref().map(|uri| uri.value.clone()),
            attributes: mandos_esdt
                .attributes
                .as_ref()
                .map(|attributes| attributes.value.clone())
                .unwrap_or_default(),
        },
    }
}

fn update_block_info(
    block_info: &mut CrateBlockInfo,
    mandos_block_info: &mandos::model::BlockInfo,
) {
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
