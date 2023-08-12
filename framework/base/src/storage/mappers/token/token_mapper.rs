use crate::{
    api::{CallTypeApi, ErrorApiImpl, StorageMapperApi},
    contract_base::BlockchainWrapper,
    esdt::ESDTSystemSmartContractProxy,
    storage::StorageKey,
    storage_get, storage_get_len, storage_set,
    types::{
        CallbackClosure, ContractCall, EsdtLocalRole, EsdtTokenPayment, ManagedAddress, ManagedRef,
        ManagedVec, TokenIdentifier,
    },
};

use super::TokenMapperState;

pub(crate) const TOKEN_ID_ALREADY_SET_ERR_MSG: &[u8] = b"Token ID already set";
pub(crate) const PENDING_ERR_MSG: &[u8] = b"Issue was already called";
pub(crate) const MUST_SET_TOKEN_ID_ERR_MSG: &[u8] = b"Must issue or set token ID first";
pub(crate) const INVALID_TOKEN_ID_ERR_MSG: &[u8] = b"Invalid token ID";
pub(crate) const INVALID_PAYMENT_TOKEN_ERR_MSG: &[u8] = b"Invalid payment token";

pub trait StorageTokenWrapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn get_storage_key(&self) -> ManagedRef<SA, StorageKey<SA>>;

    fn is_empty(&self) -> bool {
        storage_get_len(self.get_storage_key()) == 0
    }

    fn get_token_state(&self) -> TokenMapperState<SA>;

    fn get_token_id(&self) -> TokenIdentifier<SA>;

    fn get_token_id_ref(&self) -> &TokenIdentifier<SA>;

    fn set_token_id(&mut self, token_id: TokenIdentifier<SA>);

    fn set_if_empty(&mut self, token_id: TokenIdentifier<SA>) {
        if self.is_empty() {
            self.set_token_id(token_id);
        }
    }

    fn require_issued_or_set(&self) {
        if self.is_empty() {
            SA::error_api_impl().signal_error(MUST_SET_TOKEN_ID_ERR_MSG);
        }
    }

    fn require_same_token(&self, expected_token_id: &TokenIdentifier<SA>) {
        let actual_token_id = self.get_token_id_ref();
        if actual_token_id != expected_token_id {
            SA::error_api_impl().signal_error(INVALID_PAYMENT_TOKEN_ERR_MSG);
        }
    }

    fn require_all_same_token(&self, payments: &ManagedVec<SA, EsdtTokenPayment<SA>>) {
        let actual_token_id = self.get_token_id_ref();
        for p in payments {
            if actual_token_id != &p.token_identifier {
                SA::error_api_impl().signal_error(INVALID_PAYMENT_TOKEN_ERR_MSG);
            }
        }
    }

    fn set_local_roles(
        &self,
        roles: &[EsdtLocalRole],
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        let own_sc_address = Self::get_sc_address();
        self.set_local_roles_for_address(&own_sc_address, roles, opt_callback);
    }

    fn set_local_roles_for_address(
        &self,
        address: &ManagedAddress<SA>,
        roles: &[EsdtLocalRole],
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.require_issued_or_set();

        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = self.get_token_id_ref();
        let mut async_call = system_sc_proxy
            .set_special_roles(address, token_id, roles[..].iter().cloned())
            .async_call();

        if let Some(cb) = opt_callback {
            async_call = async_call.with_callback(cb);
        }

        async_call.call_and_exit()
    }

    fn get_sc_address() -> ManagedAddress<SA> {
        let b_wrapper = BlockchainWrapper::new();
        b_wrapper.get_sc_address()
    }
}

pub(crate) fn store_token_id<
    SA: StorageMapperApi + CallTypeApi,
    Mapper: StorageTokenWrapper<SA>,
>(
    mapper: &Mapper,
    token_id: &TokenIdentifier<SA>,
) {
    if mapper.get_token_state().is_set() {
        SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
    }
    if !token_id.is_valid_esdt_identifier() {
        SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
    }
    storage_set(
        mapper.get_storage_key(),
        &TokenMapperState::Token(token_id.clone()),
    );
}
pub(crate) fn check_not_set<SA: StorageMapperApi + CallTypeApi, Mapper: StorageTokenWrapper<SA>>(
    mapper: &Mapper,
) {
    let storage_value: TokenMapperState<SA> = storage_get(mapper.get_storage_key());
    match storage_value {
        TokenMapperState::NotSet => {},
        TokenMapperState::Pending => {
            SA::error_api_impl().signal_error(PENDING_ERR_MSG);
        },
        TokenMapperState::Token(_) => {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        },
    }
}
