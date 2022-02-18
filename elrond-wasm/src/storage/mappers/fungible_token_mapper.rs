use elrond_codec::{EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput};

use super::StorageMapper;
use crate::{
    api::{BlockchainApiImpl, CallTypeApi, ErrorApiImpl, StorageMapperApi},
    contract_base::{BlockchainWrapper, SendWrapper},
    esdt::{ESDTSystemSmartContractProxy, FungibleTokenProperties},
    storage::StorageKey,
    storage_get, storage_get_len, storage_set,
    types::{
        BigUint, CallbackClosure, EsdtLocalRole, EsdtTokenPayment, ManagedAddress, ManagedBuffer,
        ManagedType, TokenIdentifier,
    },
};

const FUNGIBLE_ISSUE_CALLBACK_NAME: &[u8] = b"default_fungible_issue_cb";
const FUNGIBLE_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME: &[u8] = b"default_fungible_init_supply_cb";
const TOKEN_ID_ALREADY_SET_ERR_MSG: &[u8] = b"Token ID already set";
const MUST_SET_TOKEN_ID_ERR_MSG: &[u8] = b"Must issue or set token ID first";
const INVALID_TOKEN_ID: &[u8] = b"Invalid token ID";

pub struct FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self { key: base_key }
    }
}

impl<SA> FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    /// Important: If you use custom callback, remember to save the token ID in the callback!
    /// If you want to use default callbacks, import the default_issue_callbacks::DefaultIssueCallbacksModule from elrond-wasm-modules
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
        if !self.is_empty() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }

        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let callback = match opt_callback {
            Some(cb) => cb,
            None => self.default_callback_closure_obj(&initial_supply),
        };

        system_sc_proxy
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: false,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .async_call()
            .with_callback(callback)
            .call_and_exit();
    }

    fn default_callback_closure_obj(&self, initial_supply: &BigUint<SA>) -> CallbackClosure<SA> {
        let initial_caller =
            ManagedAddress::<SA>::from_raw_handle(SA::blockchain_api_impl().get_caller_handle());
        let cb_name = if initial_supply > &0 {
            FUNGIBLE_ISSUE_WITH_INIT_SUPPLY_CALLBACK_NAME
        } else {
            FUNGIBLE_ISSUE_CALLBACK_NAME
        };

        let mut cb_closure = CallbackClosure::new(cb_name.into());
        cb_closure.push_endpoint_arg(&initial_caller);
        cb_closure.push_endpoint_arg(&self.key.buffer);

        cb_closure
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
        if self.is_empty() {
            SA::error_api_impl().signal_error(MUST_SET_TOKEN_ID_ERR_MSG);
        }

        let system_sc_proxy = ESDTSystemSmartContractProxy::<SA>::new_proxy_obj();
        let token_id = self.get_token_id();
        let mut async_call = system_sc_proxy
            .set_special_roles(address, &token_id, roles[..].iter().cloned())
            .async_call();

        if let Some(cb) = opt_callback {
            async_call = async_call.with_callback(cb);
        }

        async_call.call_and_exit();
    }

    pub fn is_empty(&self) -> bool {
        storage_get_len(self.key.as_ref()) == 0
    }

    pub fn get_token_id(&self) -> TokenIdentifier<SA> {
        storage_get(self.key.as_ref())
    }

    pub fn set_token_id(&self, token_id: &TokenIdentifier<SA>) {
        if !self.is_empty() {
            SA::error_api_impl().signal_error(TOKEN_ID_ALREADY_SET_ERR_MSG);
        }
        if !token_id.is_valid_esdt_identifier() {
            SA::error_api_impl().signal_error(INVALID_TOKEN_ID);
        }

        storage_set(self.key.as_ref(), token_id);
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
        let send_wrapper = SendWrapper::<SA>::new();

        send_wrapper.direct(to, &payment.token_identifier, 0, &payment.amount, &[]);

        payment
    }

    pub fn burn(&self, amount: &BigUint<SA>) {
        let send_wrapper = SendWrapper::<SA>::new();
        let token_id = self.get_token_id();

        send_wrapper.esdt_local_burn(&token_id, 0, &amount);
    }

    pub fn get_current_sc_balance(&self) -> BigUint<SA> {
        let own_sc_address = Self::get_sc_address();
        let token_id = self.get_token_id();

        SA::blockchain_api_impl().get_esdt_balance(&own_sc_address, &token_id, 0)
    }

    fn get_sc_address() -> ManagedAddress<SA> {
        let b_wrapper = BlockchainWrapper::new();
        b_wrapper.get_sc_address()
    }
}

impl<SA> TopEncodeMulti for FungibleTokenMapper<SA>
where
    SA: StorageMapperApi + CallTypeApi,
{
    type DecodeAs = TokenIdentifier<SA>;

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
