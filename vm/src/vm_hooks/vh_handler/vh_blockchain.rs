use crate::{
    types::{EsdtLocalRole, EsdtLocalRoleFlags, RawHandle, VMAddress},
    vm_hooks::VMHooksHandlerSource,
    world_mock::{EsdtData, EsdtInstance},
};
use num_bigint::BigInt;
use num_traits::Zero;

// The Go VM doesn't do it, but if we change that, we can enable it easily here too via this constant.
const ESDT_TOKEN_DATA_FUNC_RESETS_VALUES: bool = false;

pub trait VMHooksBlockchain: VMHooksHandlerSource {
    fn is_contract_address(&self, address_bytes: &[u8]) -> bool {
        let address = VMAddress::from_slice(address_bytes);
        &address == self.current_address()
    }

    fn managed_caller(&self, dest_handle: RawHandle) {
        self.m_types_lock()
            .mb_set(dest_handle, self.input_ref().from.to_vec());
    }

    fn managed_sc_address(&self, dest_handle: RawHandle) {
        self.m_types_lock()
            .mb_set(dest_handle, self.current_address().to_vec());
    }

    fn managed_owner_address(&self, dest_handle: RawHandle) {
        self.m_types_lock().mb_set(
            dest_handle,
            self.current_account_data()
                .contract_owner
                .unwrap_or_else(|| panic!("contract owner address not set"))
                .to_vec(),
        );
    }

    fn get_shard_of_address(&self, address_bytes: &[u8]) -> i32 {
        (address_bytes[address_bytes.len() - 1] % 3).into()
    }

    fn is_smart_contract(&self, address_bytes: &[u8]) -> bool {
        VMAddress::from_slice(address_bytes).is_smart_contract_address()
    }

    fn load_balance(&self, address_bytes: &[u8], dest: RawHandle) {
        assert!(
            self.is_contract_address(address_bytes),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        self.m_types_lock()
            .bi_overwrite(dest, self.current_account_data().egld_balance.into());
    }

    fn get_tx_hash(&self, dest: RawHandle) {
        self.m_types_lock()
            .mb_set(dest, self.input_ref().tx_hash.to_vec());
    }

    fn get_gas_left(&self) -> u64 {
        self.input_ref().gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.get_current_block_info().block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.get_current_block_info().block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.get_current_block_info().block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.get_current_block_info().block_epoch
    }

    fn get_block_random_seed(&self, dest: RawHandle) {
        self.m_types_lock().mb_set(
            dest,
            self.get_current_block_info().block_random_seed.to_vec(),
        );
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.get_previous_block_info().block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.get_previous_block_info().block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.get_previous_block_info().block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.get_previous_block_info().block_epoch
    }

    fn get_prev_block_random_seed(&self, dest: RawHandle) {
        self.m_types_lock().mb_set(
            dest,
            self.get_previous_block_info().block_random_seed.to_vec(),
        );
    }

    fn get_current_esdt_nft_nonce(&self, address_bytes: &[u8], token_id_bytes: &[u8]) -> u64 {
        assert!(
            self.is_contract_address(address_bytes),
            "get_current_esdt_nft_nonce not yet implemented for accounts other than the contract itself"
        );

        self.current_account_data()
            .esdt
            .get_by_identifier_or_default(token_id_bytes)
            .last_nonce
    }

    fn big_int_get_esdt_external_balance(
        &self,
        address_bytes: &[u8],
        token_id_bytes: &[u8],
        nonce: u64,
        dest: RawHandle,
    ) {
        assert!(
            self.is_contract_address(address_bytes),
            "get_esdt_balance not yet implemented for accounts other than the contract itself"
        );

        let esdt_balance = self
            .current_account_data()
            .esdt
            .get_esdt_balance(token_id_bytes, nonce);
        self.m_types_lock().bi_overwrite(dest, esdt_balance.into());
    }

    #[allow(clippy::too_many_arguments)]
    fn managed_get_esdt_token_data(
        &self,
        address_handle: RawHandle,
        token_id_handle: RawHandle,
        nonce: u64,
        value_handle: RawHandle,
        properties_handle: RawHandle,
        hash_handle: RawHandle,
        name_handle: RawHandle,
        attributes_handle: RawHandle,
        creator_handle: RawHandle,
        royalties_handle: RawHandle,
        uris_handle: RawHandle,
    ) {
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        let account = self.account_data(&address);

        if let Some(esdt_data) = account.esdt.get_by_identifier(token_id_bytes.as_slice()) {
            if let Some(instance) = esdt_data.instances.get_by_nonce(nonce) {
                self.set_esdt_data_values(
                    esdt_data,
                    instance,
                    value_handle,
                    properties_handle,
                    hash_handle,
                    name_handle,
                    attributes_handle,
                    creator_handle,
                    royalties_handle,
                    uris_handle,
                )
            } else {
                // missing nonce
                self.reset_esdt_data_values(
                    value_handle,
                    properties_handle,
                    hash_handle,
                    name_handle,
                    attributes_handle,
                    creator_handle,
                    royalties_handle,
                    uris_handle,
                );
            }
        } else {
            // missing token identifier
            self.reset_esdt_data_values(
                value_handle,
                properties_handle,
                hash_handle,
                name_handle,
                attributes_handle,
                creator_handle,
                royalties_handle,
                uris_handle,
            );
        }
    }

    fn check_esdt_frozen(
        &self,
        address_handle: RawHandle,
        token_id_handle: RawHandle,
        _nonce: u64,
    ) -> bool {
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        let account = self.account_data(&address);
        if let Some(esdt_data) = account.esdt.get_by_identifier(token_id_bytes.as_slice()) {
            return esdt_data.frozen;
        }

        false
    }

    fn get_esdt_local_roles_bits(&self, token_id_handle: RawHandle) -> u64 {
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        let account = self.current_account_data();
        let mut result = EsdtLocalRoleFlags::NONE;
        if let Some(esdt_data) = account.esdt.get_by_identifier(token_id_bytes.as_slice()) {
            for role_name in esdt_data.roles.get() {
                result |= EsdtLocalRole::from(role_name.as_slice()).to_flag();
            }
        }
        result.bits()
    }

    #[allow(clippy::too_many_arguments)]
    fn set_esdt_data_values(
        &self,
        esdt_data: &EsdtData,
        instance: &EsdtInstance,
        value_handle: RawHandle,
        properties_handle: RawHandle,
        hash_handle: RawHandle,
        name_handle: RawHandle,
        attributes_handle: RawHandle,
        creator_handle: RawHandle,
        royalties_handle: RawHandle,
        uris_handle: RawHandle,
    ) {
        let mut m_types = self.m_types_lock();
        m_types.bi_overwrite(value_handle, instance.balance.clone().into());
        if esdt_data.frozen {
            m_types.mb_set(properties_handle, vec![1, 0]);
        } else {
            m_types.mb_set(properties_handle, vec![0, 0]);
        }
        m_types.mb_set(
            hash_handle,
            instance.metadata.hash.clone().unwrap_or_default(),
        );
        m_types.mb_set(name_handle, instance.metadata.name.clone());
        m_types.mb_set(attributes_handle, instance.metadata.attributes.clone());
        if let Some(creator) = &instance.metadata.creator {
            m_types.mb_set(creator_handle, creator.to_vec());
        } else {
            m_types.mb_set(creator_handle, vec![0u8; 32]);
        };
        m_types.bi_overwrite(
            royalties_handle,
            num_bigint::BigInt::from(instance.metadata.royalties),
        );
        m_types.mb_set_vec_of_bytes(uris_handle, instance.metadata.uri.clone());
    }

    #[allow(clippy::too_many_arguments)]
    fn reset_esdt_data_values(
        &self,
        value_handle: RawHandle,
        properties_handle: RawHandle,
        hash_handle: RawHandle,
        name_handle: RawHandle,
        attributes_handle: RawHandle,
        creator_handle: RawHandle,
        royalties_handle: RawHandle,
        uris_handle: RawHandle,
    ) {
        if ESDT_TOKEN_DATA_FUNC_RESETS_VALUES {
            let mut m_types = self.m_types_lock();
            m_types.bi_overwrite(value_handle, BigInt::zero());
            m_types.mb_set(properties_handle, vec![0, 0]);
            m_types.mb_set(hash_handle, vec![]);
            m_types.mb_set(name_handle, vec![]);
            m_types.mb_set(attributes_handle, vec![]);
            m_types.mb_set(creator_handle, vec![0u8; 32]);
            m_types.bi_overwrite(royalties_handle, BigInt::zero());
            m_types.bi_overwrite(uris_handle, BigInt::zero());
        }
    }
}
