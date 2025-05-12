use crate::{
    blockchain::state::{EsdtData, EsdtInstance},
    chain_core::builtin_func_names::*,
    host::vm_hooks::VMHooksHandlerSource,
    types::{EsdtLocalRole, EsdtLocalRoleFlags, RawHandle, VMAddress},
};
use multiversx_chain_core::types::ReturnCode;
use multiversx_chain_vm_executor::VMHooksEarlyExit;
use num_bigint::BigInt;
use num_traits::Zero;

// The Go VM doesn't do it, but if we change that, we can enable it easily here too via this constant.
const ESDT_TOKEN_DATA_FUNC_RESETS_VALUES: bool = false;
const VM_BUILTIN_FUNCTION_NAMES: [&str; 16] = [
    ESDT_LOCAL_MINT_FUNC_NAME,
    ESDT_LOCAL_BURN_FUNC_NAME,
    ESDT_MULTI_TRANSFER_FUNC_NAME,
    ESDT_NFT_TRANSFER_FUNC_NAME,
    ESDT_NFT_CREATE_FUNC_NAME,
    ESDT_NFT_ADD_QUANTITY_FUNC_NAME,
    ESDT_NFT_ADD_URI_FUNC_NAME,
    ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME,
    ESDT_NFT_BURN_FUNC_NAME,
    ESDT_TRANSFER_FUNC_NAME,
    CHANGE_OWNER_BUILTIN_FUNC_NAME,
    CLAIM_DEVELOPER_REWARDS_FUNC_NAME,
    SET_USERNAME_FUNC_NAME,
    MIGRATE_USERNAME_FUNC_NAME,
    DELETE_USERNAME_FUNC_NAME,
    UPGRADE_CONTRACT_FUNC_NAME,
];

pub trait VMHooksBlockchain: VMHooksHandlerSource {
    fn is_contract_address(&mut self, address_bytes: &[u8]) -> Result<bool, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.is_smart_contract)?;

        let address = VMAddress::from_slice(address_bytes);
        Ok(&address == self.current_address())
    }

    fn managed_caller(&mut self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        self.m_types_lock()
            .mb_set(dest_handle, self.input_ref().from.to_vec());
        Ok(())
    }

    fn managed_sc_address(&mut self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        self.m_types_lock()
            .mb_set(dest_handle, self.current_address().to_vec());
        Ok(())
    }

    fn managed_owner_address(&mut self, dest_handle: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        self.m_types_lock().mb_set(
            dest_handle,
            self.current_account_data()
                .contract_owner
                .unwrap_or_else(|| panic!("contract owner address not set"))
                .to_vec(),
        );
        Ok(())
    }

    fn get_shard_of_address(&mut self, address_bytes: &[u8]) -> Result<i32, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_shard_of_address)?;

        Ok((address_bytes[address_bytes.len() - 1] % 3).into())
    }

    fn is_smart_contract(&mut self, address_bytes: &[u8]) -> Result<bool, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.is_smart_contract)?;

        Ok(VMAddress::from_slice(address_bytes).is_smart_contract_address())
    }

    fn load_balance(
        &mut self,
        address_bytes: &[u8],
        dest: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        assert!(
            self.is_contract_address(address_bytes)?,
            "get balance not yet implemented for accounts other than the contract itself"
        );
        self.m_types_lock()
            .bi_overwrite(dest, self.current_account_data().egld_balance.into());

        Ok(())
    }

    fn get_tx_hash(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_current_tx_hash)?;

        self.m_types_lock()
            .mb_set(dest, self.input_ref().tx_hash.to_vec());
        Ok(())
    }

    fn get_gas_left(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_gas_left)?;

        Ok(self.input_ref().gas_limit as i64)
    }

    fn get_block_timestamp(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_time_stamp)?;

        Ok(self.get_current_block_info().block_timestamp as i64)
    }

    fn get_block_nonce(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_nonce)?;

        Ok(self.get_current_block_info().block_nonce as i64)
    }

    fn get_block_round(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_round)?;

        Ok(self.get_current_block_info().block_round as i64)
    }

    fn get_block_epoch(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_epoch)?;

        Ok(self.get_current_block_info().block_epoch as i64)
    }

    fn get_block_random_seed(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_random_seed)?;

        self.m_types_lock().mb_set(
            dest,
            self.get_current_block_info().block_random_seed.to_vec(),
        );
        Ok(())
    }

    fn get_prev_block_timestamp(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_time_stamp)?;

        Ok(self.get_previous_block_info().block_timestamp as i64)
    }

    fn get_prev_block_nonce(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_nonce)?;

        Ok(self.get_previous_block_info().block_nonce as i64)
    }

    fn get_prev_block_round(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_round)?;

        Ok(self.get_previous_block_info().block_round as i64)
    }

    fn get_prev_block_epoch(&mut self) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_epoch)?;

        Ok(self.get_previous_block_info().block_epoch as i64)
    }

    fn get_prev_block_random_seed(&mut self, dest: RawHandle) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.get_block_random_seed)?;

        self.m_types_lock().mb_set(
            dest,
            self.get_previous_block_info().block_random_seed.to_vec(),
        );
        Ok(())
    }

    fn get_current_esdt_nft_nonce(
        &mut self,
        address_bytes: &[u8],
        token_id_bytes: &[u8],
    ) -> Result<i64, VMHooksEarlyExit> {
        assert!(
            self.is_contract_address(address_bytes)?,
            "get_current_esdt_nft_nonce not yet implemented for accounts other than the contract itself"
        );

        Ok(self
            .current_account_data()
            .esdt
            .get_by_identifier_or_default(token_id_bytes)
            .last_nonce as i64)
    }

    fn big_int_get_esdt_external_balance(
        &mut self,
        address_bytes: &[u8],
        token_id_bytes: &[u8],
        nonce: u64,
        dest: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        assert!(
            self.is_contract_address(address_bytes)?,
            "get_esdt_balance not yet implemented for accounts other than the contract itself"
        );

        let esdt_balance = self
            .current_account_data()
            .esdt
            .get_esdt_balance(token_id_bytes, nonce);
        self.m_types_lock().bi_overwrite(dest, esdt_balance.into());

        Ok(())
    }

    fn managed_get_code_metadata(
        &mut self,
        address_handle: i32,
        response_handle: i32,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;

        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let Some(data) = self.account_data(&address) else {
            return Err(
                VMHooksEarlyExit::new(ReturnCode::ExecutionFailed.as_u64()).with_message(format!(
                    "account not found: {}",
                    hex::encode(address.as_bytes())
                )),
            );
        };
        let code_metadata_bytes = data.code_metadata.to_byte_array();
        self.m_types_lock()
            .mb_set(response_handle, code_metadata_bytes.to_vec());

        Ok(())
    }

    fn managed_is_builtin_function(
        &mut self,
        function_name_handle: i32,
    ) -> Result<bool, VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().base_ops_api_cost.is_builtin_function)?;

        Ok(VM_BUILTIN_FUNCTION_NAMES.contains(
            &self
                .m_types_lock()
                .mb_to_function_name(function_name_handle)
                .as_str(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn managed_get_esdt_token_data(
        &mut self,
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
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;
        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();

        if let Some(account) = self.account_data(&address) {
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
                    )?
                }
            }
        }

        // missing account/token identifier/nonce
        self.reset_esdt_data_values(
            value_handle,
            properties_handle,
            hash_handle,
            name_handle,
            attributes_handle,
            creator_handle,
            royalties_handle,
            uris_handle,
        )?;

        Ok(())
    }

    fn managed_get_back_transfers(
        &mut self,
        esdt_transfer_value_handle: RawHandle,
        call_value_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;

        let (call_value, esdt_transfers_len, esdt_transfers) = {
            let back_transfers = self.back_transfers_lock();

            (
                back_transfers.call_value.clone(),
                back_transfers.esdt_transfers.len(),
                back_transfers.esdt_transfers.clone(),
            )
        };

        self.use_gas(self.calculate_set_vec_of_esdt_transfers_gas_cost(esdt_transfers_len)?)?;

        let mut m_types = self.m_types_lock();

        m_types.bi_overwrite(call_value_handle, call_value.into());
        let num_bytes_copied =
            m_types.mb_set_vec_of_esdt_payments(esdt_transfer_value_handle, &esdt_transfers);
        std::mem::drop(m_types);
        self.use_gas_for_data_copy(num_bytes_copied)?;

        Ok(())
    }

    fn check_esdt_frozen(
        &mut self,
        address_handle: RawHandle,
        token_id_handle: RawHandle,
        _nonce: u64,
    ) -> Result<bool, VMHooksEarlyExit> {
        self.use_gas(
            2 * self
                .gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;

        let address = VMAddress::from_slice(self.m_types_lock().mb_get(address_handle));
        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        if let Some(account) = self.account_data(&address) {
            if let Some(esdt_data) = account.esdt.get_by_identifier(token_id_bytes.as_slice()) {
                return Ok(esdt_data.frozen);
            }
        }

        // Might be better to return Err and check
        Ok(false)
    }

    fn get_esdt_local_roles_bits(
        &mut self,
        token_id_handle: RawHandle,
    ) -> Result<i64, VMHooksEarlyExit> {
        self.use_gas(
            self.gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_get_bytes,
        )?;

        let token_id_bytes = self.m_types_lock().mb_get(token_id_handle).to_vec();
        let account = self.current_account_data();
        let mut result = EsdtLocalRoleFlags::NONE;
        if let Some(esdt_data) = account.esdt.get_by_identifier(token_id_bytes.as_slice()) {
            for role_name in esdt_data.roles.get() {
                result |= EsdtLocalRole::from(role_name.as_slice()).to_flag();
            }
        }
        Ok(result.bits() as i64)
    }

    #[allow(clippy::too_many_arguments)]
    fn set_esdt_data_values(
        &mut self,
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
    ) -> Result<(), VMHooksEarlyExit> {
        self.use_gas(2 * self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;
        self.use_gas(
            5 * self
                .gas_schedule()
                .managed_buffer_api_cost
                .m_buffer_set_bytes,
        )?;
        self.use_gas(self.calculate_set_vec_of_bytes_gas_cost(instance.metadata.uri.len())?)?;

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

        let num_bytes_copied =
            m_types.mb_set_vec_of_bytes(uris_handle, instance.metadata.uri.clone());
        std::mem::drop(m_types);
        self.use_gas_for_data_copy(num_bytes_copied)?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn reset_esdt_data_values(
        &mut self,
        value_handle: RawHandle,
        properties_handle: RawHandle,
        hash_handle: RawHandle,
        name_handle: RawHandle,
        attributes_handle: RawHandle,
        creator_handle: RawHandle,
        royalties_handle: RawHandle,
        uris_handle: RawHandle,
    ) -> Result<(), VMHooksEarlyExit> {
        if ESDT_TOKEN_DATA_FUNC_RESETS_VALUES {
            self.use_gas(3 * self.gas_schedule().big_int_api_cost.big_int_set_int_64)?;
            self.use_gas(
                5 * self
                    .gas_schedule()
                    .managed_buffer_api_cost
                    .m_buffer_set_bytes,
            )?;

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

        Ok(())
    }
}
