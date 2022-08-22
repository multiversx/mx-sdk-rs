use crate::mandos_system::model::{SetStateStep, Step};
use elrond_wasm::types::heap::Address;

use crate::world_mock::{
    is_smart_contract_address, AccountData, AccountEsdt, BlockInfo as CrateBlockInfo,
    BlockchainMock, EsdtData, EsdtInstance, EsdtInstanceMetadata, EsdtInstances, EsdtRoles,
};

impl BlockchainMock {
    pub fn mandos_set_state(&mut self, set_state_step: SetStateStep) -> &mut Self {
        execute(self, &set_state_step);
        self.mandos_trace.steps.push(Step::SetState(set_state_step));
        self
    }
}

fn execute(state: &mut BlockchainMock, set_state_step: &SetStateStep) {
    for (address, account) in set_state_step.accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let esdt = AccountEsdt::new_from_raw_map(
            account
                .esdt
                .iter()
                .map(|(k, v)| (k.value.clone(), convert_mandos_esdt_to_world_mock(v)))
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.to_address(),
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
                .map(|address_value| address_value.value.clone()),
            developer_rewards: account
                .developer_rewards
                .as_ref()
                .map(|rewards| rewards.value.clone())
                .unwrap_or_default(),
        });
    }
    for new_address in set_state_step.new_addresses.iter() {
        assert!(
            is_smart_contract_address(&new_address.new_address.value),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.value.clone(),
            new_address.creator_nonce.value,
            new_address.new_address.value.clone(),
        )
    }
    if let Some(block_info_obj) = &*set_state_step.previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*set_state_step.current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn convert_mandos_esdt_to_world_mock(mandos_esdt: &crate::mandos_system::model::Esdt) -> EsdtData {
    match mandos_esdt {
        crate::mandos_system::model::Esdt::Short(short_esdt) => {
            let balance = short_esdt.value.clone();
            let mut esdt_data = EsdtData::default();
            esdt_data.instances.add(0, balance);
            esdt_data
        },
        crate::mandos_system::model::Esdt::Full(full_esdt) => EsdtData {
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
                    .map(|role| role.as_bytes().to_vec())
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
    mandos_esdt: &crate::mandos_system::model::EsdtInstance,
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
            uri: mandos_esdt
                .uri
                .iter()
                .map(|uri| uri.value.clone())
                .collect(),
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
    mandos_block_info: &crate::mandos_system::model::BlockInfo,
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
