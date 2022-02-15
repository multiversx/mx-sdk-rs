#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2018::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use elrond_wasm::{
    api::{
        BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl, CryptoApi,
        CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi, LogApiImpl,
        ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
    },
    arrayvec::ArrayVec,
    contract_base::{ContractBase, ProxyObjBase},
    elrond_codec::{DecodeError, NestedDecode, NestedEncode, TopDecode},
    err_msg,
    esdt::*,
    io::*,
    non_zero_usize,
    non_zero_util::*,
    require, require_old, sc_error, sc_panic, sc_print,
    storage::mappers::*,
    types::{
        SCResult::{Err, Ok},
        *,
    },
    Box, Vec,
};
use elrond_wasm::{
    derive::{ManagedVecItem, TypeAbi},
    elrond_codec,
    elrond_codec::elrond_codec_derive::{
        NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
    },
};
const EGLD_NUM_DECIMALS: usize = 18;
/// Converts between EGLD and a wrapped EGLD ESDT token.
///	1 EGLD = 1 wrapped EGLD and is interchangeable at all times.
/// Also manages the supply of wrapped EGLD tokens.
pub trait EgldEsdtSwap: elrond_wasm::contract_base::ContractBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(&self) {}
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_wrapped_egld(
        &self,
        token_display_name: elrond_wasm::types::ManagedBuffer<Self::Api>,
        token_ticker: elrond_wasm::types::ManagedBuffer<Self::Api>,
    ) -> elrond_wasm::types::AsyncCall<Self::Api> {
        if (!(self.wrapped_egld_token_id().is_empty())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "wrapped egld was already issued",
            );
        };
        let issue_cost = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();
        let initial_supply = elrond_wasm::types::BigUint::<Self::Api>::zero();
        self.issue_started_event(&caller, &token_ticker, &initial_supply);
        self.send()
            .esdt_system_sc_proxy()
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: EGLD_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().esdt_issue_callback(&caller))
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn esdt_issue_callback(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        result: ManagedAsyncCallResult<Self::Api, ()>,
    ) {
        let (returned_tokens, token_identifier) = self.call_value().payment_token_pair();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.wrapped_egld_token_id().set(&token_identifier);
            }
            ManagedAsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, &message.err_msg);
                if token_identifier.is_egld() && returned_tokens > 0 {
                    self.send().direct_egld(caller, &returned_tokens, &[]);
                }
            }
        }
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn set_local_roles(&self) -> elrond_wasm::types::AsyncCall<Self::Api> {
        if (!(!self.wrapped_egld_token_id().is_empty())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Must issue token first",
            );
        };
        let roles = [EsdtLocalRole::Mint, EsdtLocalRole::Burn];
        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                &self.blockchain().get_sc_address(),
                &self.wrapped_egld_token_id().get(),
                roles[..].iter().cloned(),
            )
            .async_call()
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrap_egld(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        if (!(payment_token.is_egld())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Only EGLD accepted",
            );
        };
        if (!(payment_amount > 0u32)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Payment must be more than 0",
            );
        };
        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
        self.send()
            .esdt_local_mint(&wrapped_egld_token_id, 0, &payment_amount);
        let caller = self.blockchain().get_caller();
        self.send()
            .direct(&caller, &wrapped_egld_token_id, 0, &payment_amount, &[]);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn unwrap_egld(&self) {
        let (payment_amount, payment_token) = self.call_value().payment_token_pair();
        let wrapped_egld_token_id = self.wrapped_egld_token_id().get();
        if (!(payment_token == wrapped_egld_token_id)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Wrong esdt token",
            );
        };
        if (!(payment_amount > 0u32)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Must pay more than 0 tokens!",
            );
        };
        if (!(payment_amount <= self.get_locked_egld_balance())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Contract does not have enough funds",
            );
        };
        self.send()
            .esdt_local_burn(&wrapped_egld_token_id, 0, &payment_amount);
        let caller = self.blockchain().get_caller();
        self.send().direct_egld(&caller, &payment_amount, &[]);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn get_locked_egld_balance(&self) -> elrond_wasm::types::BigUint<Self::Api> {
        self.blockchain()
            .get_sc_balance(&elrond_wasm::types::TokenIdentifier::<Self::Api>::egld(), 0)
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrapped_egld_token_id(
        &self,
    ) -> SingleValueMapper<Self::Api, elrond_wasm::types::TokenIdentifier<Self::Api>>;
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_started_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        token_ticker: &elrond_wasm::types::ManagedBuffer<Self::Api>,
        initial_supply: &elrond_wasm::types::BigUint<Self::Api>,
    );
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_success_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        token_identifier: &elrond_wasm::types::TokenIdentifier<Self::Api>,
        initial_supply: &elrond_wasm::types::BigUint<Self::Api>,
    );
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_failure_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        message: &elrond_wasm::types::ManagedBuffer<Self::Api>,
    );
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrap_egld_event(
        &self,
        user: &elrond_wasm::types::ManagedAddress<Self::Api>,
        amount: &elrond_wasm::types::BigUint<Self::Api>,
    );
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn unwrap_egld_event(
        &self,
        user: &elrond_wasm::types::ManagedAddress<Self::Api>,
        amount: &elrond_wasm::types::BigUint<Self::Api>,
    );
    fn callbacks(&self) -> self::CallbackProxyObj<Self::Api>;
}
pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
impl<C> EgldEsdtSwap for C
where
    C: AutoImpl,
{
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrapped_egld_token_id(
        &self,
    ) -> SingleValueMapper<Self::Api, elrond_wasm::types::TokenIdentifier<Self::Api>> {
        let mut ___key___ =
            elrond_wasm::storage::StorageKey::<Self::Api>::new(&b"wrappedEgldTokenId"[..]);
        < SingleValueMapper < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Api > > :: new (___key___)
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_started_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        token_ticker: &elrond_wasm::types::ManagedBuffer<Self::Api>,
        initial_supply: &elrond_wasm::types::BigUint<Self::Api>,
    ) {
        let mut ___topic_accumulator___ = elrond_wasm::log_util::event_topic_accumulator::<Self::Api>(
            &[
                105u8, 115u8, 115u8, 117u8, 101u8, 45u8, 115u8, 116u8, 97u8, 114u8, 116u8, 101u8,
                100u8,
            ][..],
        );
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, caller);
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, token_ticker);
        let ___data_buffer___ = elrond_wasm::log_util::serialize_log_data(initial_supply);
        elrond_wasm::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_success_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        token_identifier: &elrond_wasm::types::TokenIdentifier<Self::Api>,
        initial_supply: &elrond_wasm::types::BigUint<Self::Api>,
    ) {
        let mut ___topic_accumulator___ = elrond_wasm::log_util::event_topic_accumulator::<Self::Api>(
            &[
                105u8, 115u8, 115u8, 117u8, 101u8, 45u8, 115u8, 117u8, 99u8, 99u8, 101u8, 115u8,
                115u8,
            ][..],
        );
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, caller);
        elrond_wasm::log_util::serialize_event_topic(
            &mut ___topic_accumulator___,
            token_identifier,
        );
        let ___data_buffer___ = elrond_wasm::log_util::serialize_log_data(initial_supply);
        elrond_wasm::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_failure_event(
        &self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
        message: &elrond_wasm::types::ManagedBuffer<Self::Api>,
    ) {
        let mut ___topic_accumulator___ = elrond_wasm::log_util::event_topic_accumulator::<Self::Api>(
            &[
                105u8, 115u8, 115u8, 117u8, 101u8, 45u8, 102u8, 97u8, 105u8, 108u8, 117u8, 114u8,
                101u8,
            ][..],
        );
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, caller);
        let ___data_buffer___ = elrond_wasm::log_util::serialize_log_data(message);
        elrond_wasm::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrap_egld_event(
        &self,
        user: &elrond_wasm::types::ManagedAddress<Self::Api>,
        amount: &elrond_wasm::types::BigUint<Self::Api>,
    ) {
        let mut ___topic_accumulator___ = elrond_wasm::log_util::event_topic_accumulator::<Self::Api>(
            &[119u8, 114u8, 97u8, 112u8, 45u8, 101u8, 103u8, 108u8, 100u8][..],
        );
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, user);
        let ___data_buffer___ = elrond_wasm::log_util::serialize_log_data(amount);
        elrond_wasm::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn unwrap_egld_event(
        &self,
        user: &elrond_wasm::types::ManagedAddress<Self::Api>,
        amount: &elrond_wasm::types::BigUint<Self::Api>,
    ) {
        let mut ___topic_accumulator___ = elrond_wasm::log_util::event_topic_accumulator::<Self::Api>(
            &[
                117u8, 110u8, 119u8, 114u8, 97u8, 112u8, 45u8, 101u8, 103u8, 108u8, 100u8,
            ][..],
        );
        elrond_wasm::log_util::serialize_event_topic(&mut ___topic_accumulator___, user);
        let ___data_buffer___ = elrond_wasm::log_util::serialize_log_data(amount);
        elrond_wasm::log_util::write_log(&___topic_accumulator___, &___data_buffer___);
    }
    fn callbacks(&self) -> self::CallbackProxyObj<Self::Api> {
        < self :: CallbackProxyObj < Self :: Api > as elrond_wasm :: contract_base :: CallbackProxyObjBase > :: new_cb_proxy_obj ()
    }
}
pub trait EndpointWrappers: elrond_wasm::contract_base::ContractBase + EgldEsdtSwap {
    #[inline]
    fn call_init(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.init();
    }
    #[inline]
    fn call_issue_wrapped_egld(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        let _ = elrond_wasm::contract_base::CallValueWrapper::<Self::Api>::new().require_egld();
        self.blockchain().check_caller_is_owner();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            2i32,
        );
        let token_display_name = elrond_wasm::load_single_arg::<
            Self::Api,
            elrond_wasm::types::ManagedBuffer<Self::Api>,
        >(0i32, ArgId::from(&b"token_display_name"[..]));
        let token_ticker = elrond_wasm::load_single_arg::<
            Self::Api,
            elrond_wasm::types::ManagedBuffer<Self::Api>,
        >(1i32, ArgId::from(&b"token_ticker"[..]));
        let result = self.issue_wrapped_egld(token_display_name, token_ticker);
        elrond_wasm::io::EndpointResult::finish::<Self::Api>(&result);
    }
    #[inline]
    fn call_set_local_roles(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        self.blockchain().check_caller_is_owner();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        let result = self.set_local_roles();
        elrond_wasm::io::EndpointResult::finish::<Self::Api>(&result);
    }
    #[inline]
    fn call_wrap_egld(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        let _ = elrond_wasm::contract_base::CallValueWrapper::<Self::Api>::new().require_egld();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.wrap_egld();
    }
    #[inline]
    fn call_unwrap_egld(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.unwrap_egld();
    }
    #[inline]
    fn call_get_locked_egld_balance(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        let result = self.get_locked_egld_balance();
        elrond_wasm::io::EndpointResult::finish::<Self::Api>(&result);
    }
    #[inline]
    fn call_wrapped_egld_token_id(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        let result = self.wrapped_egld_token_id();
        elrond_wasm::io::EndpointResult::finish::<Self::Api>(&result);
    }
    fn call(&self, fn_name: &[u8]) -> bool {
        if match fn_name {
            b"callBack"
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self::EndpointWrappers::callback(self);
                return true;
            }
            b"init"
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::ViewContract,
                ) =>
            {
                elrond_wasm::external_view_contract::external_view_contract_constructor::<Self::Api>(
                );
                return true;
            }
            [105u8, 110u8, 105u8, 116u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_init();
                true
            }
            [105u8, 115u8, 115u8, 117u8, 101u8, 87u8, 114u8, 97u8, 112u8, 112u8, 101u8, 100u8, 69u8, 103u8, 108u8, 100u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_issue_wrapped_egld();
                true
            }
            [115u8, 101u8, 116u8, 76u8, 111u8, 99u8, 97u8, 108u8, 82u8, 111u8, 108u8, 101u8, 115u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_set_local_roles();
                true
            }
            [119u8, 114u8, 97u8, 112u8, 69u8, 103u8, 108u8, 100u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_wrap_egld();
                true
            }
            [117u8, 110u8, 119u8, 114u8, 97u8, 112u8, 69u8, 103u8, 108u8, 100u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_unwrap_egld();
                true
            }
            [103u8, 101u8, 116u8, 76u8, 111u8, 99u8, 107u8, 101u8, 100u8, 69u8, 103u8, 108u8, 100u8, 66u8, 97u8, 108u8, 97u8, 110u8, 99u8, 101u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_get_locked_egld_balance();
                true
            }
            [103u8, 101u8, 116u8, 87u8, 114u8, 97u8, 112u8, 112u8, 101u8, 100u8, 69u8, 103u8, 108u8, 100u8, 84u8, 111u8, 107u8, 101u8, 110u8, 73u8, 100u8, 101u8, 110u8, 116u8, 105u8, 102u8, 105u8, 101u8, 114u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_wrapped_egld_token_id();
                true
            }
            other => false,
        } {
            return true;
        }
        false
    }
    fn callback_selector(
        &self,
        mut ___cb_closure___: elrond_wasm::types::CallbackClosureForDeser<Self::Api>,
    ) -> elrond_wasm::types::CallbackSelectorResult<Self::Api> {
        let mut ___call_result_loader___ =
            elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();
        let ___cb_closure_matcher___ = ___cb_closure___.matcher::<32usize>();
        if ___cb_closure_matcher___.matches_empty() {
            return elrond_wasm::types::CallbackSelectorResult::Processed;
        } else if ___cb_closure_matcher___.name_matches(b"esdt_issue_callback") {
            let mut ___cb_arg_loader___ = ___cb_closure___.into_arg_loader();
            let caller: &elrond_wasm::types::ManagedAddress<Self::Api> =
                &elrond_wasm::load_dyn_arg(&mut ___cb_arg_loader___, ArgId::from(&b"caller"[..]));
            let result: ManagedAsyncCallResult<Self::Api, ()> = elrond_wasm::load_dyn_arg(
                &mut ___call_result_loader___,
                ArgId::from(&b"result"[..]),
            );
            ___cb_arg_loader___.assert_no_more_args();
            ___call_result_loader___.assert_no_more_args();
            self.esdt_issue_callback(&caller, result);
            return elrond_wasm::types::CallbackSelectorResult::Processed;
        }
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
    }
    fn callback(&self) {
        if let Some(___cb_closure___) =
            elrond_wasm::types::CallbackClosureForDeser::storage_load_and_clear::<Self::Api>()
        {
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_closure___)
            {
                elrond_wasm::api::ErrorApiImpl::signal_error(
                    &Self::Api::error_api_impl(),
                    err_msg::CALLBACK_BAD_FUNC,
                );
            }
        }
    }
}
pub struct AbiProvider {}
impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
    type Api = elrond_wasm::api::uncallable::UncallableApi;
    fn abi() -> elrond_wasm::abi::ContractAbi {
        let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "egld-esdt-swap" , version : "0.0.0" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & ["Converts between EGLD and a wrapped EGLD ESDT token." , "\\t1 EGLD = 1 wrapped EGLD and is interchangeable at all times." , "Also manages the supply of wrapped EGLD tokens."] , name : "EgldEsdtSwap" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : true , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "init",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        contract_abi.constructors.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "issueWrappedEgld",
            only_owner: true,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["EGLD"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi
            .add_input::<elrond_wasm::types::ManagedBuffer<Self::Api>>("token_display_name");
        contract_abi.add_type_descriptions::<elrond_wasm::types::ManagedBuffer<Self::Api>>();
        endpoint_abi.add_input::<elrond_wasm::types::ManagedBuffer<Self::Api>>("token_ticker");
        contract_abi.add_type_descriptions::<elrond_wasm::types::ManagedBuffer<Self::Api>>();
        endpoint_abi.add_output::<elrond_wasm::types::AsyncCall<Self::Api>>(&[]);
        contract_abi.add_type_descriptions::<elrond_wasm::types::AsyncCall<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "setLocalRoles",
            only_owner: true,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<elrond_wasm::types::AsyncCall<Self::Api>>(&[]);
        contract_abi.add_type_descriptions::<elrond_wasm::types::AsyncCall<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "wrapEgld",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["EGLD"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "unwrapEgld",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "getLockedEgldBalance",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Readonly,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<elrond_wasm::types::BigUint<Self::Api>>(&[]);
        contract_abi.add_type_descriptions::<elrond_wasm::types::BigUint<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "getWrappedEgldTokenIdentifier",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Readonly,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi . add_output :: < SingleValueMapper < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > > (& []) ;
        contract_abi . add_type_descriptions :: < SingleValueMapper < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > > () ;
        contract_abi.endpoints.push(endpoint_abi);
        contract_abi
    }
}
pub struct ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    _phantom: core::marker::PhantomData<A>,
}
impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    type Api = A;
}
impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    fn call(&self, fn_name: &[u8]) -> bool {
        EndpointWrappers::call(self, fn_name)
    }
    fn clone_obj(&self) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
        elrond_wasm::Box::new(ContractObj::<A> {
            _phantom: core::marker::PhantomData,
        })
    }
}
pub fn contract_obj<A>() -> ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    ContractObj {
        _phantom: core::marker::PhantomData,
    }
}
pub struct ContractBuilder;
impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
    fn new_contract_obj<A: elrond_wasm::api::VMApi>(
        &self,
    ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
        elrond_wasm::Box::new(ContractObj::<A> {
            _phantom: core::marker::PhantomData,
        })
    }
}
#[allow(non_snake_case)]
pub mod endpoints {
    use super::EndpointWrappers;
    pub fn init<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_init();
    }
    pub fn issueWrappedEgld<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_issue_wrapped_egld();
    }
    pub fn setLocalRoles<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_set_local_roles();
    }
    pub fn wrapEgld<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_wrap_egld();
    }
    pub fn unwrapEgld<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_unwrap_egld();
    }
    pub fn getLockedEgldBalance<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_get_locked_egld_balance();
    }
    pub fn getWrappedEgldTokenIdentifier<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_wrapped_egld_token_id();
    }
    pub fn callBack<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().callback();
    }
}
pub trait ProxyTrait: elrond_wasm::contract_base::ProxyObjBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(self) -> elrond_wasm::types::ContractDeploy<Self::Api> {
        let ___address___ = self.into_fields();
        let mut ___contract_deploy___ =
            elrond_wasm::types::new_contract_deploy::<Self::Api>(___address___);
        ___contract_deploy___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn issue_wrapped_egld(
        self,
        token_display_name: elrond_wasm::types::ManagedBuffer<Self::Api>,
        token_ticker: elrond_wasm::types::ManagedBuffer<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <elrond_wasm::types::AsyncCall<Self::Api> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"issueWrappedEgld"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___.push_endpoint_arg(token_display_name);
        ___contract_call___.push_endpoint_arg(token_ticker);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn set_local_roles(
        self,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <elrond_wasm::types::AsyncCall<Self::Api> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"setLocalRoles"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn wrap_egld(
        self,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"wrapEgld"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn unwrap_egld(
        self,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"unwrapEgld"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn get_locked_egld_balance(
        self,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <elrond_wasm::types::BigUint<Self::Api> as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"getLockedEgldBalance"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]    fn wrapped_egld_token_id (self) -> elrond_wasm :: types :: ContractCall < Self :: Api , < SingleValueMapper < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > as elrond_wasm :: io :: EndpointResult > :: DecodeAs >{
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"getWrappedEgldTokenIdentifier"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___
    }
}
pub struct Proxy<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    pub address: elrond_wasm::types::ManagedAddress<A>,
}
impl<A> elrond_wasm::contract_base::ProxyObjBase for Proxy<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    type Api = A;
    fn new_proxy_obj() -> Self {
        let zero_address = ManagedAddress::zero();
        Proxy {
            address: zero_address,
        }
    }
    fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
        self.address = address;
        self
    }
    #[inline]
    fn into_fields(self) -> ManagedAddress<Self::Api> {
        self.address
    }
}
impl<A> ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}
pub struct CallbackProxyObj<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    _phantom: core::marker::PhantomData<A>,
}
impl<A> elrond_wasm::contract_base::CallbackProxyObjBase for CallbackProxyObj<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    type Api = A;
    fn new_cb_proxy_obj() -> Self {
        CallbackProxyObj {
            _phantom: core::marker::PhantomData,
        }
    }
}
pub trait CallbackProxy: elrond_wasm::contract_base::CallbackProxyObjBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn esdt_issue_callback(
        self,
        caller: &elrond_wasm::types::ManagedAddress<Self::Api>,
    ) -> elrond_wasm::types::CallbackClosure<Self::Api> {
        let mut ___callback_call___ =
            elrond_wasm::types::new_callback_call::<Self::Api>(&b"esdt_issue_callback"[..]);
        ___callback_call___.push_endpoint_arg(caller);
        ___callback_call___
    }
}
impl<A> self::CallbackProxy for CallbackProxyObj<A> where A: elrond_wasm::api::VMApi + 'static {}
