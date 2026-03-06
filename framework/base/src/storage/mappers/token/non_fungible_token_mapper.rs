use multiversx_chain_core::types::EsdtLocalRole;

use crate::{
    abi::TypeAbiFrom,
    codec::{EncodeErrorHandler, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput},
    storage::mappers::{
        StorageMapperFromAddress,
        source::{CurrentStorage, StorageAddress},
    },
    storage_clear, storage_get, storage_get_len, storage_set,
    types::{
        ESDTSystemSCAddress, EgldPayment, FunctionCall, ManagedVec, OriginalResultMarker, Tx,
        TxScEnv, system_proxy::ESDTSystemSCProxy,
    },
};

use super::{
    super::StorageMapper,
    TokenMapperState,
    error::{
        INVALID_PAYMENT_TOKEN_ERR_MSG, INVALID_TOKEN_ID_ERR_MSG, MUST_SET_TOKEN_ID_ERR_MSG,
        PENDING_ERR_MSG, TOKEN_ID_ALREADY_SET_ERR_MSG,
    },
    fungible_token_mapper::DEFAULT_ISSUE_CALLBACK_NAME,
};
use crate::{
    abi::{TypeAbi, TypeName},
    api::{CallTypeApi, ErrorApiImpl, StorageMapperApi},
    contract_base::{BlockchainWrapper, SendWrapper},
    storage::StorageKey,
    types::{
        BigUint, CallbackClosure, EsdtTokenData, EsdtTokenIdentifier, EsdtTokenPayment,
        EsdtTokenType, ManagedAddress, ManagedBuffer, ManagedType,
        system_proxy::{
            MetaTokenProperties, NonFungibleTokenProperties, SemiFungibleTokenProperties,
        },
    },
};

const INVALID_TOKEN_TYPE_ERR_MSG: &str = "Invalid token type for NonFungible issue";

/// High-level mapper for non-fungible, semi-fungible, and meta-fungible ESDT tokens.
/// Provides comprehensive NFT/SFT lifecycle management including issuance, creation,
/// minting, burning, and attribute management.
///
/// # Storage Layout
///
/// The mapper stores the token state at the base key:
/// - `base_key` → `TokenMapperState<SA>` (NotSet | Pending | Token(EsdtTokenIdentifier))
///
/// # Main Operations
///
/// ## Token Lifecycle
/// - **Issue**: Create new NFT/SFT collection via `issue()` or `issue_and_set_all_roles()`
/// - **Set ID**: Manually set token ID with `set_token_id()` for existing collections
/// - **Query**: Check token state with `is_empty()`, `get_token_id()`, etc.
///
/// ## NFT Operations
/// - **Create**: Mint new NFTs with `nft_create()` or `nft_create_named()`
/// - **Add Quantity**: Increase SFT supply with `nft_add_quantity()`
/// - **Update**: Modify NFT attributes with `nft_update_attributes()`
/// - **Burn**: Destroy NFT/SFT with `nft_burn()`
/// - **Transfer**: Send tokens with `send_payment()`
///
/// ## Token Management
/// - **Roles**: Manage collection roles with `set_local_roles()`
/// - **Balance**: Query token balance with `get_balance()`
/// - **Metadata**: Retrieve token data with `get_all_token_data()`, `get_token_attributes()`
///
/// # Trade-offs
///
/// **Advantages:**
/// - Supports all non-fungible token types (NFT, SFT, MetaFungible)
/// - Complete NFT lifecycle in one mapper
/// - Built-in metadata and attribute management
/// - Automatic nonce handling
/// - Payment validation utilities
///
/// **Limitations:**
/// - Single collection per mapper instance
/// - Requires careful callback implementation for issuance
/// - Token creation requires local roles
/// - Attribute updates limited by protocol
pub type IssueCallTo<Api> = Tx<
    TxScEnv<Api>,
    (),
    ESDTSystemSCAddress,
    EgldPayment<Api>,
    (),
    FunctionCall<Api>,
    OriginalResultMarker<EsdtTokenIdentifier<Api>>,
>;

pub struct NonFungibleTokenMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi + CallTypeApi,
    A: StorageAddress<SA>,
{
    key: StorageKey<SA>,
    token_state: TokenMapperState<SA>,
    address: A,
}

impl<SA> StorageMapper<SA> for NonFungibleTokenMapper<SA, CurrentStorage>
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

impl<SA> StorageMapperFromAddress<SA> for NonFungibleTokenMapper<SA, ManagedAddress<SA>>
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

impl<SA> NonFungibleTokenMapper<SA, CurrentStorage>
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
        token_type: EsdtTokenType,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        num_decimals: usize,
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.check_not_set();

        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(),
        };
        let contract_call = match token_type {
            EsdtTokenType::NonFungible => {
                Self::nft_issue(issue_cost, token_display_name, token_ticker)
            }
            EsdtTokenType::SemiFungible => {
                Self::sft_issue(issue_cost, token_display_name, token_ticker)
            }
            EsdtTokenType::MetaFungible => {
                Self::meta_issue(issue_cost, token_display_name, token_ticker, num_decimals)
            }
            _ => SA::error_api_impl().signal_error(INVALID_TOKEN_TYPE_ERR_MSG.as_bytes()),
        };

        storage_set(self.get_storage_key(), &TokenMapperState::<SA>::Pending);
        contract_call.with_callback(callback).async_call_and_exit();
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
        token_type: EsdtTokenType,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        num_decimals: usize,
        opt_callback: Option<CallbackClosure<SA>>,
    ) -> ! {
        self.check_not_set();

        if token_type == EsdtTokenType::Fungible || token_type == EsdtTokenType::Invalid {
            SA::error_api_impl().signal_error(INVALID_TOKEN_TYPE_ERR_MSG.as_bytes());
        }

        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(),
        };

        storage_set(self.get_storage_key(), &TokenMapperState::<SA>::Pending);
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_and_set_all_roles(
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .callback(callback)
            .async_call_and_exit()
    }

    pub fn clear(&mut self) {
        let state: TokenMapperState<SA> = storage_get(self.key.as_ref());
        if state.is_pending() {
            storage_clear(self.key.as_ref());
        }
    }

    pub fn nft_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
    ) -> IssueCallTo<SA> {
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_non_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                NonFungibleTokenProperties::default(),
            )
    }

    pub fn sft_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
    ) -> IssueCallTo<SA> {
        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .issue_semi_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                SemiFungibleTokenProperties::default(),
            )
    }

    pub fn meta_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        num_decimals: usize,
    ) -> IssueCallTo<SA> {
        let properties = MetaTokenProperties {
            num_decimals,
            ..Default::default()
        };

        Tx::new_tx_from_sc()
            .to(ESDTSystemSCAddress)
            .typed(ESDTSystemSCProxy)
            .register_meta_esdt(issue_cost, &token_display_name, &token_ticker, properties)
    }

    pub fn nft_create<T: TopEncode>(
        &self,
        amount: BigUint<SA>,
        attributes: &T,
    ) -> EsdtTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        let token_nonce = send_wrapper.esdt_nft_create_compact(&token_id, &amount, attributes);

        EsdtTokenPayment::new(token_id, token_nonce, amount)
    }

    pub fn nft_create_named<T: TopEncode>(
        &self,
        amount: BigUint<SA>,
        name: &ManagedBuffer<SA>,
        attributes: &T,
    ) -> EsdtTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        let token_nonce =
            send_wrapper.esdt_nft_create_compact_named(&token_id, &amount, name, attributes);

        EsdtTokenPayment::new(token_id, token_nonce, amount)
    }

    pub fn nft_create_and_send<T: TopEncode>(
        &self,
        to: &ManagedAddress<SA>,
        amount: BigUint<SA>,
        attributes: &T,
    ) -> EsdtTokenPayment<SA> {
        let payment = self.nft_create(amount, attributes);
        self.send_payment(to, &payment);

        payment
    }

    pub fn nft_create_and_send_named<T: TopEncode>(
        &self,
        to: &ManagedAddress<SA>,
        amount: BigUint<SA>,
        name: &ManagedBuffer<SA>,
        attributes: &T,
    ) -> EsdtTokenPayment<SA> {
        let payment = self.nft_create_named(amount, name, attributes);
        self.send_payment(to, &payment);

        payment
    }

    pub fn nft_add_quantity(&self, token_nonce: u64, amount: BigUint<SA>) -> EsdtTokenPayment<SA> {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.esdt_local_mint(&token_id, token_nonce, &amount);

        EsdtTokenPayment::new(token_id, token_nonce, amount)
    }

    pub fn nft_add_quantity_and_send(
        &self,
        to: &ManagedAddress<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> EsdtTokenPayment<SA> {
        let payment = self.nft_add_quantity(token_nonce, amount);
        self.send_payment(to, &payment);

        payment
    }

    pub fn nft_update_attributes<T: TopEncode>(&self, token_nonce: u64, new_attributes: &T) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id_ref();
        send_wrapper.nft_update_attributes(token_id, token_nonce, new_attributes);
    }

    pub fn nft_burn(&self, token_nonce: u64, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id_ref();

        send_wrapper.esdt_local_burn(token_id, token_nonce, amount);
    }

    pub fn send_payment(&self, to: &ManagedAddress<SA>, payment: &EsdtTokenPayment<SA>) {
        Tx::new_tx_from_sc()
            .to(to)
            .single_esdt(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            )
            .transfer();
    }

    pub fn set_token_id(&mut self, token_id: EsdtTokenIdentifier<SA>) {
        self.store_token_id(&token_id);
        self.token_state = TokenMapperState::Token(token_id);
    }

    pub fn set_if_empty(&mut self, token_id: EsdtTokenIdentifier<SA>) {
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

    pub(crate) fn store_token_id(&self, token_id: &EsdtTokenIdentifier<SA>) {
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

    pub fn get_balance(&self, token_nonce: u64) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_esdt_balance(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_sc_address() -> ManagedAddress<SA> {
        let b_wrapper = BlockchainWrapper::new();
        b_wrapper.get_sc_address()
    }

    pub fn get_all_token_data(&self, token_nonce: u64) -> EsdtTokenData<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_esdt_token_data(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_token_attributes<T: TopDecode>(&self, token_nonce: u64) -> T {
        let token_data = self.get_all_token_data(token_nonce);
        token_data.decode_attributes()
    }
}

impl<SA, A> NonFungibleTokenMapper<SA, A>
where
    SA: StorageMapperApi + CallTypeApi,
    A: StorageAddress<SA>,
{
    pub(crate) fn check_not_set(&self) {
        let storage_value: TokenMapperState<SA> = storage_get(self.get_storage_key());
        match storage_value {
            TokenMapperState::NotSet => {}
            TokenMapperState::Pending => {
                SA::error_api_impl().signal_error(PENDING_ERR_MSG);
            }
            TokenMapperState::Token(_) => {
                SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        storage_get_len(self.get_storage_key()) == 0
    }

    pub fn require_issued_or_set(&self) {
        if self.is_empty() {
            SA::error_api_impl().signal_error(MUST_SET_TOKEN_ID_ERR_MSG);
        }
    }

    pub fn require_same_token(&self, expected_token_id: &EsdtTokenIdentifier<SA>) {
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

    pub fn get_storage_key(&self) -> crate::types::ManagedRef<'_, SA, StorageKey<SA>> {
        self.key.as_ref()
    }

    pub fn get_token_state(&self) -> TokenMapperState<SA> {
        self.token_state.clone()
    }

    pub fn get_token_id(&self) -> EsdtTokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token.clone()
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
    }

    pub fn get_token_id_ref(&self) -> &EsdtTokenIdentifier<SA> {
        if let TokenMapperState::Token(token) = &self.token_state {
            token
        } else {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID_ERR_MSG);
        }
    }

    pub fn default_callback_closure_obj(&self) -> CallbackClosure<SA> {
        let initial_caller = BlockchainWrapper::<SA>::new().get_caller();
        let cb_name = DEFAULT_ISSUE_CALLBACK_NAME;

        let mut cb_closure = CallbackClosure::new(cb_name);
        cb_closure.push_endpoint_arg(&initial_caller);
        cb_closure.push_endpoint_arg(&self.key.buffer);

        cb_closure
    }
}

impl<SA> TopEncodeMulti for NonFungibleTokenMapper<SA>
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

impl<SA> TypeAbiFrom<NonFungibleTokenMapper<SA>> for EsdtTokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbiFrom<Self> for NonFungibleTokenMapper<SA> where SA: StorageMapperApi + CallTypeApi {}

impl<SA> TypeAbi for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        EsdtTokenIdentifier::<SA>::type_name()
    }

    fn type_name_rust() -> TypeName {
        EsdtTokenIdentifier::<SA>::type_name_rust()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        EsdtTokenIdentifier::<SA>::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        false
    }
}
