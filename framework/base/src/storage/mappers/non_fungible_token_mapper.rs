use crate::codec::{
    CodecFrom, EncodeErrorHandler, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
};

use super::{
    fungible_token_mapper::DEFAULT_ISSUE_CALLBACK_NAME,
    token_mapper::{
        read_token_id, store_token_id, StorageTokenWrapper, TOKEN_ID_ALREADY_SET_ERR_MSG,
    },
    StorageMapper,
};
use crate::{
    abi::{TypeAbi, TypeName},
    api::{CallTypeApi, ErrorApiImpl, StorageMapperApi},
    contract_base::{BlockchainWrapper, SendWrapper},
    esdt::{
        ESDTSystemSmartContractProxy, MetaTokenProperties, NonFungibleTokenProperties,
        SemiFungibleTokenProperties,
    },
    storage::StorageKey,
    types::{
        BigUint, CallbackClosure, ContractCall, ContractCallWithEgld, EsdtTokenData,
        EsdtTokenPayment, EsdtTokenType, ManagedAddress, ManagedBuffer, ManagedType,
        TokenIdentifier,
    },
};

const INVALID_TOKEN_TYPE_ERR_MSG: &[u8] = b"Invalid token type for NonFungible issue";

pub struct NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
    token_id: TokenIdentifier<SA>,
}

impl<SA> StorageMapper<SA> for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            token_id: read_token_id(base_key.as_ref()),
            key: base_key,
        }
    }
}

impl<SA> StorageTokenWrapper<SA> for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn get_storage_key(&self) -> crate::types::ManagedRef<SA, StorageKey<SA>> {
        self.key.as_ref()
    }

    fn get_token_id(&self) -> TokenIdentifier<SA> {
        self.token_id.clone()
    }

    fn get_token_id_ref(&self) -> &TokenIdentifier<SA> {
        &self.token_id
    }

    fn set_token_id(&mut self, token_id: TokenIdentifier<SA>) {
        store_token_id(self, &token_id);
        self.token_id = token_id;
    }
}

impl<SA> NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    /// Important: If you use custom callback, remember to save the token ID in the callback!
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
        if !self.is_empty() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }

        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(),
        };
        let contract_call = match token_type {
            EsdtTokenType::NonFungible => {
                Self::nft_issue(issue_cost, token_display_name, token_ticker)
            },
            EsdtTokenType::SemiFungible => {
                Self::sft_issue(issue_cost, token_display_name, token_ticker)
            },
            EsdtTokenType::Meta => {
                Self::meta_issue(issue_cost, token_display_name, token_ticker, num_decimals)
            },
            _ => SA::error_api_impl().signal_error(INVALID_TOKEN_TYPE_ERR_MSG),
        };

        contract_call
            .async_call()
            .with_callback(callback)
            .call_and_exit();
    }

    /// Important: If you use custom callback, remember to save the token ID in the callback!
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
        if !self.is_empty() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }
        if token_type == EsdtTokenType::Fungible || token_type == EsdtTokenType::Invalid {
            SA::error_api_impl().signal_error(INVALID_TOKEN_TYPE_ERR_MSG);
        }

        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(),
        };

        system_sc_proxy
            .issue_and_set_all_roles(
                issue_cost,
                token_display_name,
                token_ticker,
                token_type,
                num_decimals,
            )
            .async_call()
            .with_callback(callback)
            .call_and_exit();
    }

    fn default_callback_closure_obj(&self) -> CallbackClosure<SA> {
        let initial_caller = BlockchainWrapper::<SA>::new().get_caller();
        let cb_name = DEFAULT_ISSUE_CALLBACK_NAME;

        let mut cb_closure = CallbackClosure::new(cb_name);
        cb_closure.push_endpoint_arg(&initial_caller);
        cb_closure.push_endpoint_arg(&self.key.buffer);

        cb_closure
    }

    fn nft_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
    ) -> ContractCallWithEgld<SA, ()> {
        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        system_sc_proxy.issue_non_fungible(
            issue_cost,
            &token_display_name,
            &token_ticker,
            NonFungibleTokenProperties::default(),
        )
    }

    fn sft_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
    ) -> ContractCallWithEgld<SA, ()> {
        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        system_sc_proxy.issue_semi_fungible(
            issue_cost,
            &token_display_name,
            &token_ticker,
            SemiFungibleTokenProperties::default(),
        )
    }

    fn meta_issue(
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        num_decimals: usize,
    ) -> ContractCallWithEgld<SA, ()> {
        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let properties = MetaTokenProperties {
            num_decimals,
            ..Default::default()
        };

        system_sc_proxy.register_meta_esdt(
            issue_cost,
            &token_display_name,
            &token_ticker,
            properties,
        )
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

    pub fn get_all_token_data(&self, token_nonce: u64) -> EsdtTokenData<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_esdt_token_data(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_balance(&self, token_nonce: u64) -> BigUint<SA> {
        let b_wrapper = BlockchainWrapper::new();
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id_ref();

        b_wrapper.get_esdt_balance(&own_sc_address, token_id, token_nonce)
    }

    pub fn get_token_attributes<T: TopDecode>(&self, token_nonce: u64) -> T {
        let token_data = self.get_all_token_data(token_nonce);
        token_data.decode_attributes()
    }

    fn send_payment(&self, to: &ManagedAddress<SA>, payment: &EsdtTokenPayment<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        send_wrapper.direct_esdt(
            to,
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
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

impl<SA> CodecFrom<NonFungibleTokenMapper<SA>> for TokenIdentifier<SA> where
    SA: StorageMapperApi + CallTypeApi
{
}

impl<SA> TypeAbi for NonFungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn type_name() -> TypeName {
        TokenIdentifier::<SA>::type_name()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        TokenIdentifier::<SA>::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        false
    }
}
