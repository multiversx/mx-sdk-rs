use multiversx_chain_core::types::EsdtLocalRole;

use crate::{
    abi::{TypeAbi, TypeAbiFrom},
    api::ErrorApiImpl,
    codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput},
    storage::mappers::{set_mapper::CurrentStorage, StorageMapperFromAddress},
    storage_clear, storage_get, storage_get_len, storage_set,
    types::{
        system_proxy::{ESDTSystemSCProxy, FungibleTokenProperties},
        ESDTSystemSCAddress, Tx,
    },
};

use super::{
    super::StorageMapper,
    error::{
        INVALID_PAYMENT_TOKEN_ERR_MSG, INVALID_TOKEN_ID_ERR_MSG, MUST_SET_TOKEN_ID_ERR_MSG,
        PENDING_ERR_MSG, TOKEN_ID_ALREADY_SET_ERR_MSG,
    },
    TokenMapperState,
};
use crate::{
    abi::TypeName,
    api::{CallTypeApi, StorageMapperApi},
    contract_base::{BlockchainWrapper, SendWrapper},
    storage::StorageKey,
    types::{
        BigUint, CallbackClosure, EsdtTokenPayment, EsdtTokenType, ManagedAddress, ManagedBuffer,
        ManagedType, ManagedVec, TokenIdentifier,
    },
};

pub(crate) const DEFAULT_ISSUE_CALLBACK_NAME: &str = "default_issue_cb";
pub(crate) const DEFAULT_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME: &str =
    "default_issue_init_supply_cb";

pub struct FungibleTokenMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
    token_state: TokenMapperState<SA>,
    address: A,
}

impl<SA> StorageMapper<SA> for FungibleTokenMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            token_state: storage_get(base_key.as_ref()),
            key: base_key,
            address: CurrentStorage,
        }
    }
}

impl<SA> StorageMapperFromAddress<SA> for FungibleTokenMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        Self {
            token_state: storage_get(base_key.as_ref()),
            key: base_key,
            address,
        }
    }
}

impl<SA> FungibleTokenMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi + CallTypeApi,
{
    /// Important: If you use custom callback, remember to save the token ID in the callback and clear the mapper in case of error! Clear is unusable outside this specific case.
    ///
    /// #[callback]
    /// fn my_custom_callback(
    ///     &self,
    ///     #[call_result] result: ManagedAsyncCallResult<()>,
    /// ) {
    ///      match result {
    ///     ManagedAsyncCallResult::Ok(token_id) => {
    ///         self.fungible_token_mapper().set_token_id(token_id);
    ///     },
    ///     ManagedAsyncCallResult::Err(_) => {
    ///         self.fungible_token_mapper().clear();
    ///     },
    /// }
    ///
    /// If you want to use default callbacks, import the default_issue_callbacks::DefaultIssueCallbacksModule from multiversx-sc-modules
    /// and pass None for the opt_callback argument
    pub fn issue(
        &self,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        initial_supply: BigUint<SA>,
        num_decimals: usize,
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.check_not_set();

        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(&initial_supply),
        };
        let properties = FungibleTokenProperties {
            num_decimals,
            ..Default::default()
        };

        storage_set(self.get_storage_key(), &TokenMapperState::<SA>::Pending);
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                properties,
            )
            .callback(callback)
            .async_call_and_exit()
    }

    /// Important: If you use custom callback, remember to save the token ID in the callback and clear the mapper in case of error! Clear is unusable outside this specific case.
    ///
    /// #[callback]
    /// fn my_custom_callback(
    ///     &self,
    ///     #[call_result] result: ManagedAsyncCallResult<()>,
    /// ) {
    ///      match result {
    ///     ManagedAsyncCallResult::Ok(token_id) => {
    ///         self.fungible_token_mapper().set_token_id(token_id);
    ///     },
    ///     ManagedAsyncCallResult::Err(_) => {
    ///         self.fungible_token_mapper().clear();
    ///     },
    /// }
    ///
    /// If you want to use default callbacks, import the default_issue_callbacks::DefaultIssueCallbacksModule from multiversx-sc-modules
    /// and pass None for the opt_callback argument
    pub fn issue_and_set_all_roles(
        &self,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        num_decimals: usize,
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.check_not_set();

        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(&BigUint::zero()),
        };

        storage_set(self.get_storage_key(), &TokenMapperState::<SA>::Pending);
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_and_set_all_roles(
                issue_cost,
                token_display_name,
                token_ticker,
                EsdtTokenType::Fungible,
                num_decimals,
            )
            .callback(callback)
            .async_call_and_exit();
    }

    pub fn clear(&mut self) {
        let state: TokenMapperState<SA> = storage_get(self.key.as_ref());
        if state.is_pending() {
            storage_clear(self.key.as_ref());
        }
    }

    pub fn mint(&self, amount: BigUint<SA>) -> EsdtTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.esdt_local_mint(&token_id, 0, &amount);

        EsdtTokenPayment::new(token_id, 0, amount)
    }

    pub fn mint_and_send(
        &self,
        to: &ManagedAddress<SA>,
        amount: BigUint<SA>,
    ) -> EsdtTokenPayment<SA> {
        let payment = self.mint(amount);
        self.send_payment(to, &payment);

        payment
    }

    pub fn burn(&self, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id_ref();

        send_wrapper.esdt_local_burn(token_id, 0, amount);
    }

    pub fn send_payment(&self, to: &ManagedAddress<SA>, payment: &EsdtTokenPayment<SA>) {
        Tx::new_tx_from_sc()
            .to(to)
            .single_esdt(&payment.token_identifier, 0, &payment.amount)
            .transfer();
    }

    pub fn set_if_empty(&mut self, token_id: TokenIdentifier<SA>) {
        if self.is_empty() {
            self.set_token_id(token_id);
        }
    }

    pub fn set_local_roles(
        &self,
        roles: &[EsdtLocalRole],
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        let own_sc_address = Self::get_sc_address();
        self.set_local_roles_for_address(&own_sc_address, roles, opt_callback);
    }

    pub fn set_local_roles_for_address(
        &self,
        address: &ManagedAddress<SA>,
        roles: &[EsdtLocalRole],
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.require_issued_or_set();

        let token_id = self.get_token_id_ref();
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(address, token_id, roles[..].iter().cloned())
            .callback(opt_callback)
            .async_call_and_exit()
    }

    pub fn set_token_id(&mut self, token_id: TokenIdentifier<SA>) {
        self.store_token_id(&token_id);
        self.token_state = TokenMapperState::Token(token_id);
    }

    pub(crate) fn store_token_id(&self, token_id: &TokenIdentifier<SA>) {
        if self.get_token_state().is_set() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }
        if !token_id.is_valid_esdt_identifier() {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
        storage_set(
            self.get_storage_key(),
            &TokenMapperState::Token(token_id.clone()),
        );
    }
}

impl<SA> FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    pub fn get_storage_key(&self) -> crate::types::ManagedRef<SA, StorageKey<SA>> {
        self.key.as_ref()
    }

    pub fn get_token_state(&self) -> TokenMapperState<SA> {
        self.token_state.clone()
    }

    pub fn get_token_id(&self) -> TokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token.clone()
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG)
        }
    }

    pub fn get_token_id_ref(&self) -> &TokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
    }

    pub fn get_sc_address() -> ManagedAddress<SA> {
        let b_wrapper = BlockchainWrapper::new();
        b_wrapper.get_sc_address()
    }

    pub fn is_empty(&self) -> bool {
        storage_get_len(self.get_storage_key()) == 0
    }

    pub fn require_issued_or_set(&self) {
        if self.is_empty() {
            SA::error_api_impl().signal_error(MUST_SET_TOKEN_ID_ERR_MSG);
        }
    }

    pub fn require_same_token(&self, expected_token_id: &TokenIdentifier<SA>) {
        let actual_token_id = self.get_token_id_ref();
        if actual_token_id != expected_token_id {
            SA::error_api_impl().signal_error(INVALID_PAYMENT_TOKEN_ERR_MSG);
        }
    }

    pub fn require_all_same_token(&self, payments: &ManagedVec<SA, EsdtTokenPayment<SA>>) {
        let actual_token_id = self.get_token_id_ref();
        for p in payments {
            if actual_token_id != &p.token_identifier {
                SA::error_api_impl().signal_error(INVALID_PAYMENT_TOKEN_ERR_MSG);
            }
        }
    }

    pub fn default_callback_closure_obj(
        &self,
        initial_supply: &BigUint<SA>,
    ) -> CallbackClosure<SA> {
        let initial_caller = BlockchainWrapper::<SA>::new().get_caller();
        let cb_name = if initial_supply > &0 {
            DEFAULT_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME
        } else {
            DEFAULT_ISSUE_CALLBACK_NAME
        };

        let mut cb_closure = CallbackClosure::new(cb_name);
        cb_closure.push_endpoint_arg(&initial_caller);
        cb_closure.push_endpoint_arg(&self.key.buffer);

        cb_closure
    }

    pub fn get_balance(&self) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_esdt_balance(&own_sc_address, token_id, 0)
    }

    pub(crate) fn check_not_set(&self) {
        let storage_value: TokenMapperState<SA> = storage_get(self.get_storage_key());
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
}

impl<SA> TopEncodeMulti for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        if self.is_empty() {
            output.push_single_value(&ManagedBuffer::<SA>::new(), h)
        } else {
            output.push_single_value(&self.get_token_id(), h)
        }
    }
}

impl<SA> TypeAbiFrom<FungibleTokenMapper<SA>> for TokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbiFrom<Self> for FungibleTokenMapper<SA> where SA: StorageMapperApi + CallTypeApi {}

impl<SA> TypeAbi for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        TokenIdentifier::<SA>::type_name()
    }

    fn type_name_rust() -> TypeName {
        TokenIdentifier::<SA>::type_name_rust()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        TokenIdentifier::<SA>::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        false
    }
}
