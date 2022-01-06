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
    only_owner, require, sc_error, signal_error,
    storage::mappers::*,
    types::{
        SCResult::{Err, Ok},
        *,
    },
    Box, Vec,
};
pub trait FormattedMessageFeatures: elrond_wasm::contract_base::ContractBase + Sized {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(&self) {}
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn dynamic_message(&self, bytes: elrond_wasm::types::ManagedBuffer<Self::Api>) {
        {
            let mut ___buffer___ =
                elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
            let mut ___encoded_arg___ = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
            ___buffer___.append_bytes(b"Got this buffer:");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&bytes, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b". I don\'t like it, ERROR!");
            let mut ___as_managed_buffer___ = ___buffer___.into_managed_buffer();
            Self::Api::error_api_impl()
                .signal_error_from_buffer(___as_managed_buffer___.get_raw_handle());
        };
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn dynamic_message_multiple(
        &self,
        token_id: elrond_wasm::types::TokenIdentifier<Self::Api>,
        nonce: u64,
        amount: elrond_wasm::types::BigUint<Self::Api>,
    ) {
        {
            let mut ___buffer___ =
                elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
            let mut ___encoded_arg___ = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
            ___buffer___.append_bytes(b"Got token");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&token_id, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b", with nonce");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&nonce, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b", amount");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&amount, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b". I prefer EGLD. ERROR!");
            let mut ___as_managed_buffer___ = ___buffer___.into_managed_buffer();
            Self::Api::error_api_impl()
                .signal_error_from_buffer(___as_managed_buffer___.get_raw_handle());
        };
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn dynamic_message_ascii(
        &self,
        token_id: elrond_wasm::types::TokenIdentifier<Self::Api>,
        nonce: u64,
        amount: elrond_wasm::types::BigUint<Self::Api>,
    ) {
        {
            let mut ___buffer___ =
                elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
            let mut ___encoded_arg___ = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
            ___buffer___.append_bytes(b"Got token");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&token_id, &mut ___encoded_arg___)
                .unwrap();
            ___buffer___.append_managed_buffer(&___encoded_arg___);
            ___buffer___.append_bytes(b", with nonce");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&nonce, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b", amount");
            elrond_wasm::elrond_codec::TopEncode::top_encode(&amount, &mut ___encoded_arg___)
                .unwrap();
            elrond_wasm::hex_util::add_arg_as_hex_to_buffer(&mut ___buffer___, ___encoded_arg___);
            ___buffer___.append_bytes(b". I prefer EGLD. ERROR!");
            let mut ___as_managed_buffer___ = ___buffer___.into_managed_buffer();
            Self::Api::error_api_impl()
                .signal_error_from_buffer(___as_managed_buffer___.get_raw_handle());
        };
    }
}
pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
impl<C> FormattedMessageFeatures for C where C: AutoImpl {}
pub trait EndpointWrappers:
    elrond_wasm::contract_base::ContractBase + FormattedMessageFeatures
{
    #[inline]
    fn call_init(&self) {
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.init();
    }
    #[inline]
    fn call_dynamic_message(&self) {
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            1i32,
        );
        let bytes = elrond_wasm::load_single_arg::<
            Self::Api,
            elrond_wasm::types::ManagedBuffer<Self::Api>,
        >(0i32, ArgId::from(&b"bytes"[..]));
        self.dynamic_message(bytes);
    }
    #[inline]
    fn call_dynamic_message_multiple(&self) {
        let (amount, token_id) = elrond_wasm::api::CallValueApiImpl::payment_token_pair(
            &Self::Api::call_value_api_impl(),
        );
        let nonce = self.call_value().esdt_token_nonce();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.dynamic_message_multiple(token_id, nonce, amount);
    }
    #[inline]
    fn call_dynamic_message_ascii(&self) {
        let (amount, token_id) = elrond_wasm::api::CallValueApiImpl::payment_token_pair(
            &Self::Api::call_value_api_impl(),
        );
        let nonce = self.call_value().esdt_token_nonce();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        self.dynamic_message_ascii(token_id, nonce, amount);
    }
    fn call(&self, fn_name: &[u8]) -> bool {
        if match fn_name {
            b"callBack" => {
                self::EndpointWrappers::callback(self);
                return true;
            }
            [105u8, 110u8, 105u8, 116u8] => {
                self.call_init();
                true
            }
            [100u8, 121u8, 110u8, 97u8, 109u8, 105u8, 99u8, 95u8, 109u8, 101u8, 115u8, 115u8, 97u8, 103u8, 101u8] =>
            {
                self.call_dynamic_message();
                true
            }
            [100u8, 121u8, 110u8, 97u8, 109u8, 105u8, 99u8, 95u8, 109u8, 101u8, 115u8, 115u8, 97u8, 103u8, 101u8, 95u8, 109u8, 117u8, 108u8, 116u8, 105u8, 112u8, 108u8, 101u8] =>
            {
                self.call_dynamic_message_multiple();
                true
            }
            [100u8, 121u8, 110u8, 97u8, 109u8, 105u8, 99u8, 95u8, 109u8, 101u8, 115u8, 115u8, 97u8, 103u8, 101u8, 95u8, 97u8, 115u8, 99u8, 105u8, 105u8] =>
            {
                self.call_dynamic_message_ascii();
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
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
    }
    fn callback(&self) {}
}
pub struct AbiProvider {}
impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
    type Api = elrond_wasm::api::uncallable::UncallableApi;
    fn abi() -> elrond_wasm::abi::ContractAbi {
        let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "formatted-message-features" , version : "0.0.0" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & [] , name : "FormattedMessageFeatures" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : false , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
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
            name: "dynamic_message",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<elrond_wasm::types::ManagedBuffer<Self::Api>>("bytes");
        contract_abi.add_type_descriptions::<elrond_wasm::types::ManagedBuffer<Self::Api>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "dynamic_message_multiple",
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
            name: "dynamic_message_ascii",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
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
impl<A> elrond_wasm::contract_base::CallableContract<A> for ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    fn call(&self, fn_name: &[u8]) -> bool {
        EndpointWrappers::call(self, fn_name)
    }
    fn clone_obj(&self) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract<A>> {
        self::contract_builder()
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
pub fn contract_builder<A>() -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract<A>>
where
    A: elrond_wasm::api::VMApi,
{
    elrond_wasm::Box::new(ContractObj {
        _phantom: core::marker::PhantomData,
    })
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
    pub fn dynamic_message<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_dynamic_message();
    }
    pub fn dynamic_message_multiple<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_dynamic_message_multiple();
    }
    pub fn dynamic_message_ascii<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_dynamic_message_ascii();
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
    fn dynamic_message(
        self,
        bytes: elrond_wasm::types::ManagedBuffer<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"dynamic_message"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___.push_endpoint_arg(bytes);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn dynamic_message_multiple(
        self,
        token_id: elrond_wasm::types::TokenIdentifier<Self::Api>,
        nonce: u64,
        amount: elrond_wasm::types::BigUint<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"dynamic_message_multiple"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___ = ___contract_call___.add_token_transfer(token_id, nonce, amount);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn dynamic_message_ascii(
        self,
        token_id: elrond_wasm::types::TokenIdentifier<Self::Api>,
        nonce: u64,
        amount: elrond_wasm::types::BigUint<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <() as elrond_wasm::io::EndpointResult>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"dynamic_message_ascii"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___ = ___contract_call___.add_token_transfer(token_id, nonce, amount);
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
