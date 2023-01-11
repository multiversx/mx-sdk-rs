use crate::{
    num_bigint,
    world_mock::{is_smart_contract_address, EsdtData, EsdtInstance},
    DebugApi,
};
use multiversx_sc::{
    api::{BlockchainApi, BlockchainApiImpl, HandleConstraints, ManagedBufferApi, ManagedTypeApi},
    types::{
        heap::{Address, H256},
        BigUint, EsdtLocalRole, EsdtLocalRoleFlags, EsdtTokenData, EsdtTokenType, ManagedAddress,
        ManagedBuffer, ManagedType, ManagedVec, TokenIdentifier,
    },
};

impl BlockchainApi for DebugApi {
    type BlockchainApiImpl = DebugApi;

    fn blockchain_api_impl() -> Self::BlockchainApiImpl {
        DebugApi::new_from_static()
    }
}

impl BlockchainApiImpl for DebugApi {
    fn get_caller_legacy(&self) -> Address {
        self.input_ref().from.clone()
    }

    fn get_sc_address_legacy(&self) -> Address {
        self.input_ref().to.clone()
    }

    fn get_owner_address_legacy(&self) -> Address {
        self.with_contract_account(|account| {
            account
                .contract_owner
                .clone()
                .unwrap_or_else(|| panic!("contract owner address not set"))
        })
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        panic!("get_shard_of_address not implemented")
    }

    fn is_smart_contract_legacy(&self, address: &Address) -> bool {
        is_smart_contract_address(address)
    }

    fn load_balance_legacy(&self, dest: Self::BigIntHandle, address: &Address) {
        assert!(
            address == &self.get_sc_address_legacy(),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        let egld_balance = self.with_contract_account(|account| account.egld_balance.clone());
        self.bi_overwrite(dest, egld_balance.into());
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        panic!("get_state_root_hash_legacy not yet implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        self.input_ref().tx_hash.clone()
    }

    fn get_gas_left(&self) -> u64 {
        self.input_ref().gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.blockchain_ref().current_block_info.block_epoch
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .current_block_info
            .block_random_seed
            .clone()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.blockchain_ref().previous_block_info.block_epoch
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_ref()
            .previous_block_info
            .block_random_seed
            .clone()
    }

    fn get_current_esdt_nft_nonce(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> u64 {
        let address = ManagedAddress::<DebugApi>::from_handle(address_handle);
        assert!(
            address.to_address() == self.get_sc_address_legacy(),
            "get_current_esdt_nft_nonce not yet implemented for accounts other than the contract itself"
        );

        self.with_contract_account(|account| {
            account
                .esdt
                .get_by_identifier_or_default(
                    TokenIdentifier::<DebugApi>::from_handle(token_id_handle)
                        .to_boxed_bytes()
                        .as_slice(),
                )
                .last_nonce
        })
    }

    fn load_esdt_balance(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        nonce: u64,
        dest: Self::BigIntHandle,
    ) {
        let address = ManagedAddress::<DebugApi>::from_handle(address_handle);
        assert!(
            address.to_address() == self.get_sc_address_legacy(),
            "get_esdt_balance not yet implemented for accounts other than the contract itself"
        );

        let esdt_balance = self.with_contract_account(|account| {
            account.esdt.get_esdt_balance(
                TokenIdentifier::<DebugApi>::from_handle(token_id_handle)
                    .to_boxed_bytes()
                    .as_slice(),
                nonce,
            )
        });
        self.bi_overwrite(dest, esdt_balance.into());
    }

    fn load_esdt_token_data<M: ManagedTypeApi>(
        &self,
        address: &ManagedAddress<M>,
        token: &TokenIdentifier<M>,
        nonce: u64,
    ) -> EsdtTokenData<M> {
        self.blockchain_cache()
            .with_account(&address.to_address(), |account| {
                let token_identifier_value = token.to_boxed_bytes();
                if let Some(esdt_data) = account
                    .esdt
                    .get_by_identifier(token_identifier_value.as_slice())
                {
                    if let Some(instance) = esdt_data.instances.get_by_nonce(nonce) {
                        self.esdt_token_data_from_instance(esdt_data, nonce, instance)
                    } else {
                        // missing nonce
                        EsdtTokenData {
                            token_type: EsdtTokenType::based_on_token_nonce(nonce),
                            ..Default::default()
                        }
                    }
                } else {
                    // missing token identifier
                    EsdtTokenData {
                        token_type: EsdtTokenType::Fungible,
                        ..Default::default()
                    }
                }
            })
    }

    fn load_esdt_token_data_unmanaged<M: ManagedTypeApi>(
        &self,
        _address: &ManagedAddress<M>,
        _token: &TokenIdentifier<M>,
        _nonce: u64,
    ) -> EsdtTokenData<M> {
        panic!("get_esdt_token_data_unmanaged is deprecated and should never be used in Rust tests")
    }

    fn check_esdt_frozen(
        &self,
        address_handle: Self::ManagedBufferHandle,
        token_id_handle: Self::ManagedBufferHandle,
        _nonce: u64,
    ) -> bool {
        let mut frozen = false;
        let address = ManagedAddress::<Self>::from_handle(address_handle).to_address();
        let token_identifier_value = self.mb_to_boxed_bytes(token_id_handle);
        self.blockchain_cache().with_account(&address, |account| {
            if let Some(esdt_data) = account
                .esdt
                .get_by_identifier(token_identifier_value.as_slice())
            {
                frozen = esdt_data.frozen;
            }
        });
        frozen
    }

    fn check_esdt_paused(&self, _token_id_handle: Self::ManagedBufferHandle) -> bool {
        false
    }

    fn check_esdt_limited_transfer(&self, _token_id_handle: Self::ManagedBufferHandle) -> bool {
        false
    }

    fn load_esdt_local_roles(
        &self,
        token_id_handle: Self::ManagedBufferHandle,
    ) -> EsdtLocalRoleFlags {
        let sc_address = self.input_ref().to.clone();
        self.blockchain_cache()
            .with_account(&sc_address, |account| {
                let mut result = EsdtLocalRoleFlags::NONE;
                if let Some(esdt_data) = account.esdt.get_by_identifier(
                    TokenIdentifier::<DebugApi>::from_handle(token_id_handle)
                        .to_boxed_bytes()
                        .as_slice(),
                ) {
                    for role_name in esdt_data.roles.get() {
                        result |= EsdtLocalRole::from(role_name.as_slice()).to_flag();
                    }
                }

                result
            })
    }
}

impl DebugApi {
    fn esdt_token_data_from_instance<M: ManagedTypeApi>(
        &self,
        esdt_data: &EsdtData,
        nonce: u64,
        instance: &EsdtInstance,
    ) -> EsdtTokenData<M> {
        let creator = if let Some(creator) = &instance.metadata.creator {
            ManagedAddress::from_address(creator)
        } else {
            ManagedAddress::zero()
        };

        let mut uris = ManagedVec::new();
        for uri in &instance.metadata.uri {
            uris.push(ManagedBuffer::new_from_bytes(uri.as_slice()));
        }

        EsdtTokenData {
            token_type: EsdtTokenType::based_on_token_nonce(nonce),
            amount: BigUint::from_handle(
                self.insert_new_big_uint(instance.balance.clone())
                    .cast_or_signal_error::<M, _>(),
            ),
            frozen: esdt_data.frozen,
            hash: ManagedBuffer::from_handle(
                self.insert_new_managed_buffer(instance.metadata.hash.clone().unwrap_or_default())
                    .cast_or_signal_error::<M, _>(),
            ),
            name: ManagedBuffer::from_handle(
                self.insert_new_managed_buffer(instance.metadata.name.clone())
                    .cast_or_signal_error::<M, _>(),
            ),
            attributes: ManagedBuffer::from_handle(
                self.insert_new_managed_buffer(instance.metadata.attributes.clone())
                    .cast_or_signal_error::<M, _>(),
            ),
            creator,
            royalties: BigUint::from_handle(
                self.insert_new_big_uint(num_bigint::BigUint::from(instance.metadata.royalties))
                    .cast_or_signal_error::<M, _>(),
            ),
            uris,
        }
    }
}
